use download_link::DownloadData;

pub struct TaskDownload {
    pub update_func: Box<dyn FnMut(DownloadData) -> () + Send>,
    pub url: String,
    pub savepath: String,
    pub save_name: String,
}

pub async fn apply(task: &TaskDownload) {
}