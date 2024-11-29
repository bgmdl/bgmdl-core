use serde::{Deserialize, Serialize};

use crate::task::model::TaskOption;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportError {
    pub error: String,
}

pub async fn apply(task: &mut ReportError, _task_option: &TaskOption) {
    log::error!("{}", task.error);
}
