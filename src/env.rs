use actix_ws::Session;
use utils::logger::LogData;


#[derive(Debug, Clone)]
pub struct DEnv {
    pub enable: bool,
    pub password: String,
    pub tool_path: String,
    pub url: String,
    pub username: String,
}

#[derive(Debug, Clone)]
pub struct Env {
    pub dblink: String,
    pub dbschema: String,
    pub download: DEnv,
    pub port: u16,
}

use std::sync::mpsc::Sender;
use crate::service::Task;

lazy_static! {
    pub static ref LOG_WS_COUNT: Mutex<i32> = Mutex::new(0);
    pub static ref LOG_WS_POLL: Mutex<Vec<(Mutex<Session>, i32)>> = Mutex::new(vec![]);
    pub static ref LOG_DATA: Mutex<Vec<LogData>> = Mutex::new(vec![]);
    pub static ref TASK_SENDER: Mutex<Sender<Task>> = Mutex::new(start_async_task_service());
    pub static ref RUNENV: Mutex<Env> = Mutex::new(Env {
        dblink: String::from(""),
        dbschema: String::from(""),
        download: DEnv {
            enable: false,
            password: String::from(""),
            tool_path: String::from(""),
            url: String::from(""),
            username: String::from(""),
        },
        port: DEFAULT_PORT,
    });
}

pub static mut DBLINK: state::InitCell<String> = state::InitCell::new();
pub static mut DBSCHEMA: state::InitCell<String> = state::InitCell::new();
