use download_link::Callback;
use lazy_static::lazy_static;
use model::{TaskDetail, TaskOption, TaskQueue};
use std::{sync::Mutex, thread};

use crate::env::DOWNLOAD_CALLBACK_FUNC_REF;

pub mod model;
pub mod run;
pub mod save;

lazy_static! {
    static ref TASK_QUEUE: Mutex<TaskQueue> = Mutex::new(TaskQueue::new());
    static ref NEW_TASKS: Mutex<Vec<(TaskDetail, TaskOption, i32)>> = Mutex::new(Vec::new());
}

pub fn add_task(task_detail: TaskDetail, task_option: TaskOption, priority: i32) {
    let mut new_tasks = NEW_TASKS.lock().unwrap();
    log::debug!(
        "task_option: {:?}, task_detail: {:?}",
        task_option,
        task_detail
    );
    new_tasks.push((task_detail, task_option, priority));
    log::debug!("add task into queue done");
}

#[allow(clippy::await_holding_lock)]
pub fn apply(callback: &'static Callback) {
    thread::spawn(move || {
        *DOWNLOAD_CALLBACK_FUNC_REF.lock().unwrap() = callback;
        async_run! {
            log::info!("task thread start");
            loop {
                let mut task_queue = TASK_QUEUE.lock().unwrap();
                if !NEW_TASKS.lock().unwrap().is_empty() {
                    log::trace!("add new task into queue");
                    let mut new_task = NEW_TASKS.lock().unwrap();
                    for (task_detail, task_option, priority) in new_task.drain(..) {
                        task_queue.push((task_detail, task_option), priority);
                    }
                }
                if task_queue.task_heap.is_empty() {
                    thread::sleep(std::time::Duration::from_millis(500));
                    continue;
                }
                log::trace!("task exec top");
                let _ = task_queue.exec_top().await;
                thread::sleep(std::time::Duration::from_millis(300));
            }
        }
    });
}
