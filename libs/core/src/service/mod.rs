use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::utils::pluginload::DownloadHandler;

use super::task;

lazy_static! {
    pub static ref DOWNLOAD_PATH: Mutex<String> = Mutex::new(String::from(""));
    pub static ref DOWNLOAD_HANDLER: Mutex<DownloadHandler> = Mutex::new({
        log::info!(
            "init download handler from {}",
            DOWNLOAD_PATH.lock().unwrap().as_str()
        );
        DownloadHandler::new(DOWNLOAD_PATH.lock().unwrap().as_str())
    });
}

pub fn start(path: &str, link: &str, username: &str, password: &str) {
    *DOWNLOAD_PATH.lock().unwrap() = String::from(path);
    lazy_static::initialize(&DOWNLOAD_HANDLER);
    DOWNLOAD_HANDLER
        .lock()
        .unwrap()
        .start(link, username, password);
    task::apply();
}
