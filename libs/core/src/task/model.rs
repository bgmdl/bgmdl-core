use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use serde::Deserialize;

use crate::declare::error::CoreError;
use super::run::{self, change_name::ChangeName, download::TaskDownload, download_all::TaskDownloadAll, report_error::ReportError};

#[derive(Deserialize, Debug, Clone)]
pub enum TaskName {
    Download,
    DownloadAll,
    ChangeName,
    ReportError,
}

#[derive(Debug, Clone)]
pub enum TaskDetail {
    Download(TaskDownload),
    DownloadAll(TaskDownloadAll),
    ChangeName(ChangeName),
    ReportError(ReportError),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct TaskMap {
    pub status: String,
    pub priopity: i32,
    pub taskid: u32,
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
    pub task_map: BinaryHeap<TaskMap>, // max in top
    pub task_list: HashMap<u32, TaskMap>,
    pub tasks: HashMap<u32, TaskDetail>, 
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
            task_map: BinaryHeap::new(),
            tasks: HashMap::new(),
            task_list: HashMap::new(),
            task_id: 0
        }
    }

    pub fn push(&mut self, task: TaskDetail, priopity: i32) {
        let taskid = self.task_id as u32;
        let item = TaskMap {
            status: "waiting".to_string(),
            priopity,
            taskid,
        };
        self.task_map.push(item.clone());
        self.task_list.insert(taskid, item);
        self.tasks.insert(taskid, task);
        self.task_id += 1;
    }

    pub fn exec_top_block(&mut self) -> Result<(), CoreError> {
        async_run!{
            self.exec_top().await
        }
    }

    pub fn change_status(&mut self, taskid: u32, status: &str) {
        if let Some(item) = self.task_list.get_mut(&taskid) {
            item.status = status.to_string();
        }
        self.task_map.retain(|task| {
            if task.taskid == taskid {
                return false;
            }
            true
        });
        self.task_map.push(self.task_list.get(&taskid).unwrap().clone());
    }

    pub async fn exec_top(&mut self) -> Result<(), CoreError> {
        log::trace!("task exec top");
        if self.task_map.is_empty() {
            log::debug!("task queue is empty");
            return Ok(());
        }
        let task = self.task_map.pop().unwrap();
        log::trace!("task exec top: {:?}", &task);
        if let Some(task) = self.tasks.get_mut(&task.taskid) {
            match task {
                TaskDetail::Download(task) => {
                    log::trace!("task exec download");
                    run::download::apply(task).await;
                }
                TaskDetail::DownloadAll(task) => {
                    log::trace!("task exec downloadall");
                    run::download_all::apply(task).await;
                }
                TaskDetail::ChangeName(task) => {
                    run::change_name::apply(task).await;
                }
                TaskDetail::ReportError(task) => {
                    run::report_error::apply(task).await;
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
        self.task_map.retain(|task| {
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

    pub fn drop_id(&mut self, taskid: u32) {
        // nlogn delete.
        self.task_map.retain(|task| task.taskid != taskid);
        self.tasks.remove(&taskid);
    }
}
