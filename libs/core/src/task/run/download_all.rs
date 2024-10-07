use download_link::DownloadData;

pub struct TaskDownloadAll {
    pub update_func: Box<dyn FnMut(DownloadData) -> () + Send>,
    pub url: String,
    pub savepath: String,
}

pub async fn apply(task: &TaskDownloadAll) {
}