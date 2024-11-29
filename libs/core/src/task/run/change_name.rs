use serde::{Deserialize, Serialize};

use crate::task::model::TaskOption;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeName {
    pub path: String,
    pub name: String,
}

pub async fn apply(_task: &mut ChangeName, _task_option: &TaskOption) {}
