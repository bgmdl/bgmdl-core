//! router: start
//! description: Start the server
//! --config -c <config>, config path (optional\, ~/.bgmdl/config.json)
//! --port -p <port>, server port (optional\, default: read by config or 1824)

use crate::{handle, utils::config_load::env_load};

pub fn run(config: Option<String>, port: Option<String>) {
    let config_path = config.unwrap_or("~/.bgmdl/config.json".to_string());
    env_load(&config_path);
    log::info!("Env loaded: {:?}", get_env!());
    core::service::start(
        get_env!(download.tool_path).as_str(),
        get_env!(download.url).as_str(),
        get_env!(download.username).as_str(),
        get_env!(download.password).as_str(),
    );
    let port = port.map(|x| x.parse::<u16>().unwrap());
    let port = port.unwrap_or(get_env!(port));
    log::info!("Starting server on port: {}", port);
    let _ = handle::main(port);
}