use std::sync::Mutex;

use download_link::Callback;
use lazy_static::lazy_static;

use crate::utils::pluginload::DownloadHandler;

use super::task;

extern "C" fn default_callback_func(_: *mut std::ffi::c_void, data: download_link::DownloadData) {
    log::info!("download progress: {}", data.progress);
    println!("download progress: {}", data.progress);
}

lazy_static! {
    pub static ref DOWNLOAD_PATH: Mutex<String> = Mutex::new(String::from(""));
    pub static ref DOWNLOAD_HANDLER: Mutex<DownloadHandler> = Mutex::new({
        log::info!(
            "init download handler from {}",
            DOWNLOAD_PATH.lock().unwrap().as_str()
        );
        DownloadHandler::new(DOWNLOAD_PATH.lock().unwrap().as_str())
    });
    pub static ref DOWNLOAD_CALLBACK_FUNC: Mutex<Callback> = Mutex::new(default_callback_func);
}

pub fn start(path: &str, link: &str, username: &str, password: &str, callback: Callback) {
    *DOWNLOAD_PATH.lock().unwrap() = String::from(path);
    lazy_static::initialize(&DOWNLOAD_HANDLER);
    DOWNLOAD_HANDLER
        .lock()
        .unwrap()
        .start(link, username, password);
    *DOWNLOAD_CALLBACK_FUNC.lock().unwrap() = callback;
    task::apply();
}
