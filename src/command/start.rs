//! router: start
//! description: Start the server
//! log_level required
//! --config -c <config>, config path (optional\, ~/.bgmdl/config.json)
//! --port -p <port>, server port (optional\, default: read by config or 1824)

use download_link::Callback;
use fern::Output;
use log::LevelFilter;
use std::str::FromStr;

use crate::{handle, model::task::callback, service::Task, utils::{config_load::env_load, logger::{setup_logger, LogData}}, TASK_SENDER};

pub fn run(log_level: String, config: Option<String>, port: Option<String>) {
    let _ = setup_logger(
        LevelFilter::from_str(log_level.as_str()).unwrap_or(LevelFilter::Info),
        Output::call(|record| {
            let _record = LogData {
                level: record.level().to_string(),
                target: record.target().to_string(),
                message: record.args().to_string(),
                time: chrono::Local::now().to_rfc3339(),
            };
            // trace do not send actix_http data.
            if  record.level() == log::Level::Trace && _record.target.starts_with("actix_http::h1") {
                return;
            }
            TASK_SENDER.lock().unwrap().send(Task::Log(crate::service::run::log::LogServiceData { log_data: _record } )).unwrap();
        })
    );
    let config_path = config.unwrap_or("~/.bgmdl/config.json".to_string());
    env_load(&config_path);
    log::info!("Env loaded: {:?}", get_env!());
    core::service::start(
        get_env!(download.tool_path).as_str(),
        get_env!(download.url).as_str(),
        get_env!(download.username).as_str(),
        get_env!(download.password).as_str(),
        &(callback as Callback)
    );
    let port = port.map(|x| x.parse::<u16>().unwrap());
    let port = port.unwrap_or(get_env!(port));
    let _ = handle::main(port);
}