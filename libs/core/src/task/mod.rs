use model::TaskQueue;

pub mod model;
pub static mut TASK_QUEUE: state::InitCell<TaskQueue> = state::InitCell::new();
pub mod save;
pub mod run;

pub fn apply() {

}