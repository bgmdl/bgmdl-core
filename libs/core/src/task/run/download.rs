use download_link::Callback;

use crate::utils::pluginload::DownloadHandler;

pub struct TaskDownload {
    pub update_func: Callback,
    pub url: String,
    pub savepath: String,
    pub save_name: String,
    pub tool_lib_path: String,
}

pub async fn apply(task: &mut TaskDownload) {
    log::info!("start to download {}", task.url);
    let handler = DownloadHandler::new(task.tool_lib_path.as_str());
    handler.download_by_link(task.url.as_str(), task.savepath.as_str(), task.save_name.as_str(), task.update_func);
}