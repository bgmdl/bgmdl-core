use crate::service::{DOWNLOAD_CALLBACK_FUNC_REF, DOWNLOAD_HANDLER};
#[derive(Debug, Clone)]
pub struct TaskDownload {
    pub taskid: i32,
    pub url: String,
    pub save_path: String,
    pub save_name: String,
    pub tool_lib_path: String,
}

#[allow(clippy::clone_on_copy)]
pub async fn apply(task: &mut TaskDownload) {
    log::info!("start to download {}", task.url);
    let callback_func = DOWNLOAD_CALLBACK_FUNC_REF.lock().unwrap();
    DOWNLOAD_HANDLER.lock().unwrap().download_by_link(
        task.taskid,
        task.url.as_str(),
        task.save_path.as_str(),
        task.save_name.as_str(),
        **callback_func,
    );
}
