// use download_link::DownloadData;

#[derive(Debug, Clone)]
pub struct TaskDownloadAll {
    pub taskid: i32,
    pub url: String,
    pub save_path: String,
}

pub async fn apply(_task: &TaskDownloadAll) {}
