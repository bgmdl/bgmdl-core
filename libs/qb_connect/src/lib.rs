
use std::{collections::HashMap, thread};

use download_link::{DownloadData, DownloadTools};
use qbit_rs::{model::{AddTorrentArg, Credential, GetTorrentListArg, TorrentFilter, TorrentSource}, Qbit};
use tokio::runtime;

pub struct QBtools {
    username: String,
    password: String,
    link: String,
    client: Option<Qbit>,
}

unsafe impl Send for QBtools {}

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

impl DownloadTools for QBtools {
    fn download_by_link(&mut self, url: &str, savepath: &str, rename: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(client) = &self.client {
            let result = async_run! {
                client.add_torrent(AddTorrentArg {
                    source: TorrentSource::Urls{urls: url.to_string().parse().unwrap()},
                    savepath: if savepath == "default".to_string() { None } else { Some(savepath.to_string()) },
                    rename: Some(rename.to_string()),
                    ..Default::default()
                }).await
            } as Result<(), qbit_rs::Error>;
            if let Err(e) = result {
                return Err(e.to_string().into());
            }
            Ok(())
        } else {
            Err("Please login first".into())
        }
    }

    fn login(&mut self, username: &str, password: &str, link: &str) -> Result<(), Box<dyn std::error::Error>> {
        let credential = Credential::new(username, password);
        let client = Qbit::new(link, credential);
        self.client = Some(client);
        self.username = username.to_string();
        self.password = password.to_string();
        self.link = link.to_string();
        Ok(())
    }

    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn progress_update_run(&mut self, mut callback: Box<dyn FnMut(DownloadData) -> () + Send>) -> () {
        let mut latest_data = HashMap::new();
        let mut data = DownloadData {
            name: "qb".to_string(),
            status: "Downloading".to_string(),
            progress: 0.0,
            speed: 0,
            eta: 0,
        };
        let username = self.username.clone();
        let password = self.password.clone();
        let link = self.link.clone();
        thread::spawn(move || {
        let credential = Credential::new(username, password);
            let client = Qbit::new(link.as_str(), credential);
            loop {
                let torrents = async_run! {
                    client.get_torrent_list(GetTorrentListArg {
                    filter: Some(TorrentFilter::Active),
                    ..Default::default()
                }).await.unwrap()};
                for torrent in torrents {
                    let progress = torrent.progress.unwrap_or(0.0);
                    let eta = torrent.eta.unwrap_or(0);
                    let speed = torrent.dlspeed.unwrap_or(0);
                    let name = torrent.name.unwrap_or("".to_string());
                    data.name = name.clone();
                    data.progress = progress;
                    data.speed = speed;
                    data.eta = eta;
                    if latest_data.contains_key(&name) {
                        let last_data: &DownloadData = latest_data.get(&name).unwrap();
                        if last_data.progress != progress || last_data.eta != eta || last_data.speed != speed {
                            latest_data.insert(name.clone(), data.clone());
                            callback(data.clone());
                        }
                    } else {
                        latest_data.insert(name.clone(), data.clone());
                        callback(data.clone());
                    }
                }
            }
        });

    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

#[no_mangle]
pub fn apply() -> *mut dyn DownloadTools {
    Box::into_raw(Box::new(QBtools {
        client: None,
        username: "".to_string(),
        password: "".to_string(),
        link: "".to_string(),
    }))
}