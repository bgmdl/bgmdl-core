use core::task::model::TaskName;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct TaskDownloadProps {
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TaskDownloadAllProps {
    pub urls: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TaskChangeNameProps {
    pub path: String,
    pub new_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TaskReportErrorProps {
    pub msg: String
}

#[derive(Deserialize, Debug, Clone)]
pub enum TaskDetailProps {
    TaskDownloadProps(TaskDownloadProps),
    TaskDownloadAllProps(TaskDownloadAllProps),
    TaskChangeNameProps(TaskChangeNameProps),
    TaskReportErrorProps(TaskReportErrorProps),
}

#[derive(Deserialize, Debug, Clone)]
pub struct TaskAddProps {
    pub task_type: TaskName,
    pub task_detail: TaskDetailProps,
}