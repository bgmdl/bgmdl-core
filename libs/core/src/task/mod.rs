use std::{sync::Mutex, thread};
use lazy_static::lazy_static;
use model::{TaskDetail, TaskQueue};

pub mod model;
pub mod save;
pub mod run;

lazy_static! {
    static ref TASK_QUEUE: Mutex<TaskQueue> = Mutex::new(TaskQueue::new());
}

pub fn add_task(task: TaskDetail, priority: i32) {
    let mut task_queue = TASK_QUEUE.lock().unwrap();
    task_queue.push(task, priority);
}

pub fn apply() {
    thread::spawn(move || {
        log::info!("task thread start");
        loop {
            let mut task_queue = TASK_QUEUE.lock().unwrap();
            if task_queue.task_map.is_empty() {
                thread::sleep(std::time::Duration::from_millis(500));
                continue;
            }
            let _ = task_queue.exec_top();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}