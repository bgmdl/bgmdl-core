use lazy_static::lazy_static;
use model::{TaskDetail, TaskQueue};
use std::{sync::Mutex, thread};

pub mod model;
pub mod run;
pub mod save;

lazy_static! {
    static ref TASK_QUEUE: Mutex<TaskQueue> = Mutex::new(TaskQueue::new());
}

pub fn add_task(task: TaskDetail, priority: i32) {
    let mut task_queue = TASK_QUEUE.lock().unwrap();
    // log::info!("add task({:?}) into queue:");
    task_queue.push(task, priority);
}

#[allow(clippy::await_holding_lock)]
pub fn apply() {
    thread::spawn(move || {
        async_run! {
            log::info!("task thread start");
            loop {
                let mut task_queue = TASK_QUEUE.lock().unwrap();
                if task_queue.task_map.is_empty() {
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
