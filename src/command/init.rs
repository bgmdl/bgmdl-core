//! router: init
//! description: Initialize the basic data.
//! --database -d <database>, database: sqlite / postgres
//! --url -u <url>, database url
//! --schema -s <schema>, schema name
//! --username -U <username>, username
//! --password -P <password>, password
//! --config -c <config>, config file path(optional\, default: ~/.bgmdl/config.json)
//! --port -p <port>, Init the server port(optional\, default: 1824)

use std::{fs, path};
use sha2::{Digest, Sha512};

pub fn ask_input(hints: &str, default: &str) -> String {
    let result = inquire::Text::new(hints).with_default(default).prompt();
    match result {
        Ok(result) => result,
        Err(err) => {
            panic!("Error: {:?}", err);
        },
    }
}

pub fn ask_password(hints: &str) -> String {
    let result = inquire::Password::new(hints).prompt();
    match result {
        Ok(result) => result,
        Err(err) => {
            panic!("Error: {:?}", err);
        },
    }
}

pub fn ask_select(hints: &str, options: Vec<&str>) -> String {
    let result = inquire::Select::new(hints, options.clone()).prompt();
    match result {
        Ok(result) => result.to_string(),
        Err(err) => {
            panic!("Error: {:?}", err);
        },
    }
}

pub fn run(url: Option<String>, database: Option<String>, schema: Option<String>, username: Option<String>, password: Option<String>, port: Option<u32>, config: Option<String>) {
    let _ = port;
    let dbtype = database.unwrap_or(ask_select("Please choose database type", vec!["sqlite", "postgres"]));
    if dbtype != "postgres" && dbtype != "sqlite" {
        log::error!("Database type not supported: {}", dbtype);
        return;
    }
    let mut url = url.unwrap_or(ask_input({
        if dbtype == "postgres" {
            "Please input database url"
        } else {
            "Please input database file path"
        }
    }, {
        if dbtype == "postgres" {
            "postgres://localhost:5432"
        } else {
            "database.db"
        }
    }));
    if dbtype == "sqlite" {
        url = format!("sqlite://{}?mode=rwc", url);
    }
    let schema = if dbtype == "postgres" {
        schema.unwrap_or(ask_input("Please input schema name", "public"))
    } else {
        "".to_string()
    };
    let username = username.unwrap_or(ask_input("Please input account username", "admin"));
    let password = base16ct::lower::encode_string(&Sha512::digest(password.unwrap_or(ask_password("Please input account password"))));
    log::info!("Initializing database on url: {}, schema: {}", url, schema);
    // Initialize the database
    let data = core::db::init::init(url.as_str(), schema.as_str(), username.as_str(), password.as_str());
    if data.is_err() {
        log::error!("Error with database: {:?}", data.err());
        return;
    }
    log::info!("Database initialized");
    let config_path = config.unwrap_or("~/.bgmdl/config.json".to_string()); //TODO: windows will not use this path.
    let config_path = path::PathBuf::from(shellexpand::tilde(&config_path).to_string());
    fs::create_dir_all(config_path.clone().parent().unwrap()).unwrap();
    let config = Json!{
        "database": {
            "url": url,
            "schema": schema,
        },
        "port": port.unwrap_or(1824),
    };
    let data = fs::write(config_path, config.to_string());
    if data.is_err() {
        log::error!("Error with config file: {:?}", data.err());
        return;
    }
    log::info!("Config file initialized");
    log::info!("Initialization completed");
}