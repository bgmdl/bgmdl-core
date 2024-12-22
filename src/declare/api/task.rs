use core::{model::task::Task, task::{model::TaskDetail, run::{change_name::ChangeName, download::TaskDownload, download_all::TaskDownloadAll, report_error::ReportError}}};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct TaskDownloadProps {
    pub url: String,
    pub save_path: String,
    pub save_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TaskDownloadAllProps {
    pub urls: String,
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
#[serde(tag = "task_type", content = "task_detail")]
pub enum TaskAddProps {
    #[serde(rename = "download")]
    Download(TaskDownloadProps),

    #[serde(rename = "download_all")]
    DownloadAll(TaskDownloadAllProps),

    #[serde(rename = "change_name")]
    ChangeName(TaskChangeNameProps),

    #[serde(rename = "report_error")]
    ReportError(TaskReportErrorProps),
}

impl From<TaskAddProps> for Task {
    fn from(props: TaskAddProps) -> Self {
        Task {
            task_detail: props.into(),
        }
    }
}

impl From<TaskAddProps> for TaskDetail {
    fn from(props: TaskAddProps) -> Self {
        match props {
            TaskAddProps::Download(props) => TaskDetail::Download(TaskDownload {
                url: props.url,
                save_path: props.save_path,
                save_name: props.save_name,
                tool_lib_path: get_env!(download.tool_path),
            }),
            TaskAddProps::DownloadAll(props) => TaskDetail::DownloadAll(TaskDownloadAll {
                url: props.urls,
                save_path: "".to_string(),
            }),
            TaskAddProps::ChangeName(props) => TaskDetail::ChangeName(ChangeName {
                path: props.path,
                name: props.new_name,
            }),
            TaskAddProps::ReportError(props) => TaskDetail::ReportError(ReportError {
                error: props.msg,
            }),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct TaskGetDetailProps {
    pub taskid: i32,
}