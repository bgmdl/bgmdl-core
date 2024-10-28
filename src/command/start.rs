//! router: start
//! description: Start the server
//! --config -c <config>, config path (optional\, ~/.bgmdl/config.json)
//! --port -p <port>, server port (optional\, default: read by config or 1824)

use serde::{Deserialize, Serialize};

use crate::{handle, DEnv, DEFAULT_PORT, RUNENV};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Database {
	#[serde(rename = "schema")]
	pub schema: Option<String>,

	#[serde(rename = "url")]
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Download {
	#[serde(rename = "enable")]
	pub enable: Option<bool>,

	#[serde(rename = "password")]
	pub password: Option<String>,

	#[serde(rename = "tool_path")]
	pub tool_path: Option<String>,

	#[serde(rename = "url")]
	pub url: Option<String>,

	#[serde(rename = "username")]
	pub username: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigJson {
	#[serde(rename = "database")]
	pub database: Option<Database>,

	#[serde(rename = "download")]
	pub download: Option<Download>,

	#[serde(rename = "port")]
	pub port: Option<u16>,
}

macro_rules! get_env {
    () => {
        RUNENV.lock().unwrap().clone()
    };
    ($field:ident) => {{
        let result = RUNENV.lock().unwrap().$field.clone();
        String::from(result)
    }};
    ($field:ident $(. $subfields:ident)*) => {{
        let result = RUNENV.lock().unwrap().$field$(.$subfields)*.clone();
        String::from(result)
    }};
}

pub fn run(config: Option<String>, port: Option<String>) {
    // load config file.
    let port = port.map(|x| x.parse::<u16>().unwrap());
    let config = config.unwrap_or("~/.bgmdl/config.json".to_string());
    log::info!("Loading config file: {}", config);
    let config = shellexpand::tilde(&config).to_string();
    let config = std::fs::read_to_string(config).unwrap();
    let config: ConfigJson = serde_json::from_str(&config).unwrap();
    let port = port.unwrap_or(config.port.unwrap_or(DEFAULT_PORT));
    log::info!("Starting server on port: {}", port);
    RUNENV.lock().unwrap().download = DEnv {
        enable: config.download.clone().unwrap().enable.unwrap_or(false),
        password: config.download.clone().unwrap().password.unwrap_or("".to_string()),
        tool_path: config.download.clone().unwrap().tool_path.unwrap_or("".to_string()),
        url: config.download.clone().unwrap().url.unwrap_or("".to_string()),
        username: config.download.clone().unwrap().username.unwrap_or("".to_string()),
    };
    RUNENV.lock().unwrap().dblink = config.database.clone().unwrap().url.unwrap_or("".to_string());
    RUNENV.lock().unwrap().dbschema = config.database.clone().unwrap().schema.unwrap_or("".to_string());
    // Start the core server
    log::info!("Env loaded: {:?}", get_env!());
    core::service::start(
        get_env!(download.tool_path).as_str(), 
        get_env!(download.url).as_str(), 
        get_env!(download.username).as_str(), 
        get_env!(download.password).as_str(),
    );
    // Start the server
    let _ = handle::main(port);
}
