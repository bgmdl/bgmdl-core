#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::await_holding_lock)]

use download_link::{init, Callback, DownloadData, LogParam};
use lazy_static::lazy_static;
use qbit_rs::{
    model::{AddTorrentArg, Credential, GetTorrentListArg, TorrentFilter, TorrentSource},
    Qbit,
};
use std::{collections::HashMap, ffi::CStr, os::raw::c_char, thread};
use tokio::runtime;

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

#[no_mangle]
pub extern "C" fn init_logger(param: LogParam) {
    init(param);
}

struct DownloadType {
    url: String,
    save_path: String,
    rename: String,
    func: Callback,
}

lazy_static! {
    static ref DOWNLOAD_TASK_STATUS: std::sync::Mutex<HashMap<String, DownloadData>> =
        std::sync::Mutex::new(HashMap::new());
    static ref DOWNLOAD_CALLBACK_TASKS: std::sync::Mutex<HashMap<String, Callback>> =
        std::sync::Mutex::new(HashMap::new());
    static ref DOWNLOAD_REQUEST: std::sync::Mutex<HashMap<String, DownloadType>> =
        std::sync::Mutex::new(HashMap::new());
}

#[no_mangle]
pub extern "C" fn start(
    link: *const c_char,
    username: *const c_char,
    password: *const c_char,
) -> i32 {
    let link = unsafe { CStr::from_ptr(link) }
        .to_str()
        .unwrap()
        .to_string();
    let username = unsafe { CStr::from_ptr(username) }
        .to_str()
        .unwrap()
        .to_string();
    let password = unsafe { CStr::from_ptr(password) }
        .to_str()
        .unwrap()
        .to_string();
    thread::spawn(move || {
        async_run! {
            log::info!("start download thread successfully");
            let mut client = Qbit::new(link.as_str(), Credential::new(username.as_str(), password.as_str()));
            let mut times = 0;
            loop {
                if times == 0 { // 3 second / update torrent status
                    // check client status.
                    let torrents = client.get_torrent_list(GetTorrentListArg {
                        filter: Some(TorrentFilter::Active),
                        ..Default::default()
                    }).await;
                    if torrents.is_err() {
                        log::warn!("cannot get torrent, trying to restart client.(sleep 5 sec)");
                        client = Qbit::new(link.as_str(), Credential::new(username.as_str(), password.as_str()));
                        thread::sleep(std::time::Duration::from_secs(5));
                        continue;
                    }
                    let mut active_torrents = vec![];
                    let torrents = torrents.unwrap();
                    log::trace!("torrents: {:?}", torrents);
                    for torrent in torrents {
                        let name = torrent.name.unwrap();
                        let progress = torrent.progress.unwrap();
                        let speed = torrent.dlspeed.unwrap();
                        let eta = torrent.eta.unwrap();
                        let data = DownloadData::new(name.as_str(), progress, speed, eta);
                        let mut status_map = DOWNLOAD_TASK_STATUS.lock().unwrap();
                        let callback_map = DOWNLOAD_CALLBACK_TASKS.lock().unwrap();
                        if status_map.contains_key(&name) {
                            let last_data = status_map.get(&name).unwrap();
                            if last_data.progress != progress || last_data.eta != eta || last_data.speed != speed {
                                status_map.insert(name.clone(), data.clone());
                                if let Some(callback) = callback_map.get(&name) {
                                    callback(std::ptr::null_mut(), data.clone());
                                }
                            }
                            active_torrents.push(name);
                        } else if callback_map.contains_key(&name) {
                            status_map.insert(name.clone(), data.clone());
                            callback_map.get(&name).unwrap()(std::ptr::null_mut(), data.clone());
                            active_torrents.push(name);
                        }
                    }
                    log::trace!("status_map: {:?}", DOWNLOAD_TASK_STATUS.lock().unwrap());
                    log::trace!("callback_map: {:?}", DOWNLOAD_CALLBACK_TASKS.lock().unwrap());
                    log::trace!("active_torrents: {:?}", active_torrents);
                    let mut status_map = DOWNLOAD_TASK_STATUS.lock().unwrap();
                    let mut callback_map = DOWNLOAD_CALLBACK_TASKS.lock().unwrap();
                    status_map.retain(|k, _| active_torrents.contains(k));
                    callback_map.retain(|k, _| active_torrents.contains(k));
                }
                // check download request
                let mut request_map = DOWNLOAD_REQUEST.lock().unwrap();
                for (name, download) in request_map.iter() {
                    log::info!("start to download: {}", name);
                    let result = client.add_torrent(AddTorrentArg {
                        source: TorrentSource::Urls{urls: download.url.to_string().parse().unwrap()},
                        savepath: if download.save_path == *"default" { None } else { Some(download.save_path.to_string()) },
                        rename: Some(download.rename.to_string()),
                        ..Default::default()
                    }).await as Result<(), qbit_rs::Error>;
                    if let Err(e) = result {
                        log::warn!("cannot add torrent: {}", e);
                    }
                    let mut callback_map = DOWNLOAD_CALLBACK_TASKS.lock().unwrap();
                    callback_map.insert(download.rename.clone(), download.func);
                }
                request_map.clear();
                times += 1;
                times %= 3;
                thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    });
    0
}

#[no_mangle]
pub extern "C" fn download_by_link(
    url: *const c_char,
    save_path: *const c_char,
    rename: *const c_char,
    callback_fn: Callback,
) -> i32 {
    let url = unsafe { CStr::from_ptr(url).to_str().unwrap().to_string() };
    let save_path = unsafe { CStr::from_ptr(save_path).to_str().unwrap().to_string() };
    let rename = unsafe { CStr::from_ptr(rename).to_str().unwrap().to_string() };
    DOWNLOAD_REQUEST.lock().unwrap().insert(
        url.clone(),
        DownloadType {
            url,
            save_path,
            rename,
            func: callback_fn,
        },
    );
    0
}
