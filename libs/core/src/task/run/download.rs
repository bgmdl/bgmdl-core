use std::os::raw::c_void;

use download_link::DownloadData;

use crate::service::DOWNLOAD_HANDLER;
#[derive(Debug, Clone)]
pub struct TaskDownload {
    pub url: String,
    pub save_path: String,
    pub save_name: String,
    pub tool_lib_path: String,
}

extern "C" fn callback(_: *mut c_void, data: DownloadData) {
    log::info!("download progress: {}", data.progress);
    println!("download progress: {}", data.progress);
}

pub async fn apply(task: &mut TaskDownload) {
    log::info!("start to download {}", task.url);
    DOWNLOAD_HANDLER.lock().unwrap().download_by_link(task.url.as_str(), task.save_path.as_str(), task.save_name.as_str(), callback);
}