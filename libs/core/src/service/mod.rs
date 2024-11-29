use super::task;
use crate::env::{DOWNLOAD_HANDLER, DOWNLOAD_PATH};
use download_link::Callback;

pub extern "C" fn default_callback_func(
    _: *mut std::ffi::c_void,
    data: download_link::DownloadData,
) {
    log::info!("download progress: {}", data.progress);
    println!("download progress: {}", data.progress);
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
