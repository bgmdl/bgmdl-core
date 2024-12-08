//! router: start
//! description: Start the server
//! log_level required
//! --config -c <config>, config path (optional\, ~/.bgmdl/config.json)
//! --port -p <port>, server port (optional\, default: read by config or 1824)

use download_link::Callback;
use fern::Output;
use log::LevelFilter;
use std::str::FromStr;

use crate::{handle, model::task::callback, utils::{config_load::env_load, logger::{setup_logger, LogData}}, LOG_DATA};

struct StoreIntoLogger;

impl From<StoreIntoLogger> for Output {
    fn from(_: StoreIntoLogger) -> Self {
        let mut outputdata = LOG_DATA.lock().unwrap();
        if outputdata.len() > 1000 {
            outputdata.remove(0);
        }
        drop(outputdata);
        Output::call(|record| {
            let mut outputdata = LOG_DATA.lock().unwrap();
            outputdata.push(LogData {
                level: record.level().to_string(),
                target: record.target().to_string(),
                message: record.args().to_string(),
                time: chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
            });
        })
    }
}

pub fn run(log_level: String, config: Option<String>, port: Option<String>) {
    let _ = setup_logger(LevelFilter::from_str(log_level.as_str()).unwrap_or(LevelFilter::Info), StoreIntoLogger);
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
    log::info!("Starting server on port: {}", port);
    let _ = handle::main(port);
}