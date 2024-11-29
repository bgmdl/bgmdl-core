// use download_link::DownloadData;

use serde::{Deserialize, Serialize};

use crate::task::model::TaskOption;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDownloadAll {
    pub url: String,
    pub save_path: String,
}

pub async fn apply(_task: &TaskDownloadAll, _task_option: &TaskOption) {}
