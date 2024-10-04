use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}};

use download_link::DownloadData;

pub enum TaskDetail {
    Download(TaskDownload),
    DownloadAll(TaskDownloadAll),
    ChangeName(ChangeName),
    ReportError(ReportError)
}

pub struct TaskDownload {
    update_func: Box<dyn FnMut(DownloadData) -> () + Send>,
    url: String,
    savepath: String,
    save_name: String,
}


pub struct TaskDownloadAll {
    update_func: Box<dyn FnMut(DownloadData) -> () + Send>,
    url: String,
    savepath: String,
}


#[derive(Clone)]
pub struct ChangeName {
    path: String,
}


#[derive(Clone)]
pub struct ReportError {
    error: String,
}

#[derive(PartialEq, Eq)]
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
    pub tasks: HashMap<u32, TaskDetail>,
}

impl TaskQueue {
    pub fn new() -> Self {
        TaskQueue {
            task_map: BinaryHeap::new(),
            tasks: HashMap::new(),
        }
    }

    pub fn push(&mut self, task: TaskDetail, priopity: i32) {
        let taskid = self.tasks.len() as u32;
        self.task_map.push(TaskMap {
            status: "waiting".to_string(),
            priopity,
            taskid
        });
        self.tasks.insert(taskid, task);
    }

    pub fn exec_top(&mut self) {
        
    }

    pub fn drop(&mut self) {
        self.tasks.retain(|taskid, task| {
            if let Some(task_map) = self.task_map.peek() {
                if task_map.taskid == *taskid {
                    return true;
                }
            }
            if let TaskDetail::Download(task) = task {
                if task.url == "done" {
                    return false;
                }
            }
            true
        });
    }

    pub fn drop_id(&mut self, taskid: u32) { // nlogn delete.
        self.task_map.retain(|task| task.taskid != taskid);
        self.tasks.remove(&taskid);
    }
}