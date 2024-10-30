use crate::{declare::config::ConfigJson, DEnv, DEFAULT_PORT, RUNENV};

pub fn env_load(config: &String) {
    log::info!("Loading config file: {}", config);
    let config = shellexpand::tilde(&config).to_string();
    let config = std::fs::read_to_string(config).unwrap();
    let config: ConfigJson = serde_json::from_str(&config).unwrap();
    RUNENV.lock().unwrap().download = DEnv {
        enable: config.download.clone().unwrap().enable.unwrap_or(false),
        password: config.download.clone().unwrap().password.unwrap_or("".to_string()),
        tool_path: config.download.clone().unwrap().tool_path.unwrap_or("".to_string()),
        url: config.download.clone().unwrap().url.unwrap_or("".to_string()),
        username: config.download.clone().unwrap().username.unwrap_or("".to_string()),
    };
    RUNENV.lock().unwrap().dblink = config.database.clone().unwrap().url.unwrap_or("".to_string());
    RUNENV.lock().unwrap().dbschema = config.database.clone().unwrap().schema.unwrap_or("".to_string());
    RUNENV.lock().unwrap().port = config.clone().port.unwrap_or(DEFAULT_PORT);
}