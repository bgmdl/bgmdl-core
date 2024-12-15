use std::sync::mpsc::{channel, Sender};
use std::thread;

pub mod run;

#[derive(Debug, Clone)]
pub enum Task {
    Log(run::log::LogServiceData)
}

pub fn start_async_task_service() -> Sender<Task>  {
    let (sender, receiver) = channel();
    thread::spawn(move || { async_run! {
        loop {
            match receiver.recv() {
                Ok(Task::Log(data)) => {
                    run::log::apply(data).await;
                }
                Err(_) => {
                    continue;
                }
            }
        }
    }});
    sender
}