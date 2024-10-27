use download_link::{DownloadData, Callback};
use qbit_rs::{model::{AddTorrentArg, Credential, GetTorrentListArg, TorrentFilter, TorrentSource}, Qbit};
use tokio::runtime;
use std::{collections::HashMap, ffi::CStr, os::raw::c_char, thread};
use lazy_static::lazy_static;
use log;

macro_rules! async_run {
    ($($body:tt)*) => {{
        let bt = runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        bt.block_on(async {
            $($body)*
        })
    }};
}

struct DownloadType {
    url: String,
    savepath: String,
    rename: String,
    func: Callback,
}

lazy_static! {
    static ref DOWNLOAD_TASK_STATUS: std::sync::Mutex<HashMap<String, DownloadData>> = std::sync::Mutex::new(HashMap::new());
    static ref DOWNLOAD_CALLBACK_TASKS: std::sync::Mutex<HashMap<String, Callback>> = std::sync::Mutex::new(HashMap::new());
    static ref DOWNLOAD_REQUEST: std::sync::Mutex<HashMap<String, DownloadType>> = std::sync::Mutex::new(HashMap::new());
}

#[no_mangle]
pub extern "C" fn start(link: *const c_char, username: *const c_char, password: *const c_char) -> i32 {
    let link = unsafe { CStr::from_ptr(link).to_str().unwrap().to_string() };
    let username = unsafe { CStr::from_ptr(username).to_str().unwrap().to_string() };
    let password = unsafe { CStr::from_ptr(password).to_str().unwrap().to_string() };
    thread::spawn(move || {
        dbg!("start download thread");
        log::info!("start download thread successfully");
        let mut client = Qbit::new(link.as_str(), Credential::new(username.as_str(), password.as_str()));
        let mut times = 0;
        loop {
            if times == 0 { // 5 second / update torrent status 
                // check client status.
                let torrents = async_run! {
                    client.get_torrent_list(GetTorrentListArg {
                    filter: Some(TorrentFilter::Active),
                    ..Default::default()
                }).await};
                if let Err(_) = torrents {
                    log::warn!("cannot get torrent, trying to restart client.(sleep 5 sec)");
                    client = Qbit::new(link.as_str(), Credential::new(username.as_str(), password.as_str()));
                    thread::sleep(std::time::Duration::from_secs(5));
                    continue;
                }
                
                let torrents = torrents.unwrap();
                for torrent in torrents {
                    dbg!(&torrent);
                    let name = torrent.name.unwrap();
                    let progress = torrent.progress.unwrap();
                    let speed = torrent.dlspeed.unwrap();
                    let eta = torrent.eta.unwrap();
                    let data = DownloadData::new(name.as_str(), progress, speed, eta);
                    let mut status_map = DOWNLOAD_TASK_STATUS.lock().unwrap();
                    let callback_map = DOWNLOAD_CALLBACK_TASKS.lock().unwrap();
                    dbg!(&status_map);
                    dbg!(&callback_map);
                    if status_map.contains_key(&name) {
                        let last_data = status_map.get(&name).unwrap();
                        if last_data.progress != progress || last_data.eta != eta || last_data.speed != speed {
                            status_map.insert(name.clone(), data.clone());
                            if let Some(callback) = callback_map.get(&name) {
                                dbg!("require callback");
                                callback(std::ptr::null_mut(), data.clone());
                            }
                        }  
                    } else if callback_map.contains_key(&name) {
                        status_map.insert(name.clone(), data.clone());
                        callback_map.get(&name).unwrap()(std::ptr::null_mut(), data.clone());
                    }
                }
            }
            // check download request
            let mut request_map = DOWNLOAD_REQUEST.lock().unwrap();
            for (name, download) in request_map.iter() {
                log::info!("start to download: {}", name);
                let result = async_run! {
                    client.add_torrent(AddTorrentArg {
                        source: TorrentSource::Urls{urls: download.url.to_string().parse().unwrap()},
                        savepath: if download.savepath == "default".to_string() { None } else { Some(download.savepath.to_string()) },
                        rename: Some(download.rename.to_string()),
                        ..Default::default()
                    }).await
                } as Result<(), qbit_rs::Error>;
                if let Err(e) = result {
                    log::warn!("cannot add torrent: {}", e);
                }
                let mut callback_map = DOWNLOAD_CALLBACK_TASKS.lock().unwrap();
                callback_map.insert(download.rename.clone(), download.func);
            }
            request_map.clear();
            times += 1;
            times %= 5;
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });
    0
}

#[no_mangle]
pub extern "C" fn download_by_link(url: *const c_char, savepath: *const c_char, rename: *const c_char, callback_fn: Callback) -> i32 {
    let url = unsafe { CStr::from_ptr(url).to_str().unwrap().to_string() };
    let savepath = unsafe { CStr::from_ptr(savepath).to_str().unwrap().to_string() };
    let rename = unsafe { CStr::from_ptr(rename).to_str().unwrap().to_string() };
    DOWNLOAD_REQUEST.lock().unwrap().insert(url.clone(), DownloadType {
        url,
        savepath,
        rename,
        func: callback_fn,
    });
    0
}