use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::run::{
    self, change_name::ChangeName, download::TaskDownload, download_all::TaskDownloadAll,
    report_error::ReportError,
};
use crate::declare::error::CoreError;

#[derive(Deserialize, Debug, Clone)]
pub enum TaskName {
    Download,
    DownloadAll,
    ChangeName,
    ReportError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    Once,
    Schedule,
}

impl From<String> for TaskType {
    fn from(task_type: String) -> Self {
        match task_type.as_str() {
            "once" => TaskType::Once,
            "schedule" => TaskType::Schedule,
            _ => TaskType::Once,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskOption {
    pub task_id: i32,
    pub tasktype: TaskType,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskDetail {
    Download(TaskDownload),
    DownloadAll(TaskDownloadAll),
    ChangeName(ChangeName),
    ReportError(ReportError),
}

impl From<TaskDownload> for TaskDetail {
    fn from(task: TaskDownload) -> Self {
        TaskDetail::Download(task)
    }
}

impl From<TaskDownloadAll> for TaskDetail {
    fn from(task: TaskDownloadAll) -> Self {
        TaskDetail::DownloadAll(task)
    }
}

impl From<ChangeName> for TaskDetail {
    fn from(task: ChangeName) -> Self {
        TaskDetail::ChangeName(task)
    }
}

impl From<ReportError> for TaskDetail {
    fn from(task: ReportError) -> Self {
        TaskDetail::ReportError(task)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct TaskMap {
    pub status: String,
    pub priopity: i32,
    pub taskid: i32,
}

impl Ord for TaskMap {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priopity.cmp(&self.priopity)
    }
}

impl PartialOrd for TaskMap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct TaskQueue {
    pub task_heap: BinaryHeap<TaskMap>, // max in top
    pub task_list: HashMap<i32, TaskMap>,
    pub tasks: HashMap<i32, (TaskDetail, TaskOption)>,
    pub task_id: usize,
}

impl Default for TaskQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskQueue {
    pub fn new() -> Self {
        TaskQueue {
            task_heap: BinaryHeap::new(),
            tasks: HashMap::new(),
            task_list: HashMap::new(),
            task_id: 0,
        }
    }

    pub fn push(&mut self, task: (TaskDetail, TaskOption), priopity: i32) {
        let (task_detail, task_option) = task;
        let item = TaskMap {
            status: "waiting".to_string(),
            priopity,
            taskid: task_option.task_id,
        };
        self.task_heap.push(item.clone());
        self.task_list.insert(task_option.task_id, item);
        self.tasks
            .insert(task_option.task_id, (task_detail, task_option));
    }

    pub fn exec_top_block(&mut self) -> Result<(), CoreError> {
        async_run! {
            self.exec_top().await
        }
    }

    pub fn change_status(&mut self, taskid: i32, status: &str) {
        if let Some(item) = self.task_list.get_mut(&taskid) {
            item.status = status.to_string();
        }
        self.task_heap.retain(|task| {
            if task.taskid == taskid {
                return false;
            }
            true
        });
        self.task_heap
            .push(self.task_list.get(&taskid).unwrap().clone());
    }

    pub async fn exec_top(&mut self) -> Result<(), CoreError> {
        log::trace!("task exec top");
        if self.task_heap.is_empty() {
            log::debug!("task queue is empty");
            return Ok(());
        }
        let task = self.task_heap.pop().unwrap();
        log::trace!("task exec top: {:?}", &task);
        if let Some((task_detail, task_option)) = self.tasks.get_mut(&task.taskid) {
            match task_detail {
                TaskDetail::Download(task_detail) => {
                    log::trace!("task exec download");
                    run::download::apply(task_detail, task_option).await;
                }
                TaskDetail::DownloadAll(task_detail) => {
                    log::trace!("task exec downloadall");
                    run::download_all::apply(task_detail, task_option).await;
                }
                TaskDetail::ChangeName(task_detail) => {
                    run::change_name::apply(task_detail, task_option).await;
                }
                TaskDetail::ReportError(task_detail) => {
                    run::report_error::apply(task_detail, task_option).await;
                }
            }
        } else {
            log::warn!("task not found in task queue (server error).");
        }
        Ok(())
    }

    pub fn drop_task(&mut self) {
        self.tasks.retain(|taskid, _task| {
            // check task status
            let item = self.task_list.get(taskid);
            if let Some(item) = item {
                if item.status == "done" {
                    return false;
                }
            } else {
                return false;
            }
            true
        });
        self.task_heap.retain(|task| {
            let item = self.task_list.get(&task.taskid);
            if item.is_none() {
                return false;
            }
            if !self.tasks.contains_key(&task.taskid) {
                return false;
            }
            true
        });
        self.task_list.retain(|taskid, _item| {
            if !self.tasks.contains_key(taskid) {
                return false;
            }
            true
        });
    }

    pub fn drop_id(&mut self, taskid: i32) {
        // nlogn delete.
        self.task_heap.retain(|task| task.taskid != taskid);
        self.tasks.remove(&taskid);
    }
}
