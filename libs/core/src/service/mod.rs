use std::sync::{Arc, Mutex};

use download_link::Callback;
use lazy_static::lazy_static;

use crate::utils::pluginload::DownloadHandler;

use super::task;

pub extern "C" fn default_callback_func(
    _: *mut std::ffi::c_void,
    data: download_link::DownloadData,
) {
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
    pub static ref DOWNLOAD_CALLBACK_FUNC_REF: Arc<Mutex<&'static Callback>> =
        Arc::new(Mutex::new(&(default_callback_func as Callback)));
}

pub fn start(path: &str, link: &str, username: &str, password: &str, callback: &'static Callback) {
    *DOWNLOAD_PATH.lock().unwrap() = String::from(path);
    lazy_static::initialize(&DOWNLOAD_HANDLER);
    DOWNLOAD_HANDLER
        .lock()
        .unwrap()
        .start(link, username, password);
    task::apply(callback);
}
