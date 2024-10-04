use model::TaskQueue;

pub mod model;
pub static mut TASK_QUEUE: state::InitCell<TaskQueue> = state::InitCell::new();
pub mod save;

pub fn apply() {

}