//! router: init
//! description: Initialize the basic data.
//! --database -d <database>, database: sqlite / postgres
//! --url -u <url>, database url
//! --schema -s <schema>, schema name
//! --username -U <username>, username
//! --password -P <password>, password
//! --config -c <config>, config file path(optional\, default: ~/.bgmdl/config.json)
//! --port -p <port>, Init the server port(optional\, default: 1824)
//! --download_username -D <download_username>, download username(optional if enable_download is false)
//! --download_password -W <download_password>, download password(optional if enable_download is false)
//! --download_url -L <download_url>, download url(optional if enable_download is false)
//! --enable_download -E <enable_download>, enable download?
//! --download_tool_path -T <download_tool_path>, download tool path(lib file) mac: .dylib\, linux: .so\, windows: .dll

use std::{fs, path};
use download_link::{DownloadFunc, StartFunc};
use crate::utils::inquire::*;

use crate::utils::encryption::encode_password;

#[allow(clippy::too_many_arguments)]
pub fn run(
    url: Option<String>,
    database: Option<String>,
    schema: Option<String>,
    username: Option<String>,
    password: Option<String>,
    port: Option<u32>,
    config: Option<String>,
    download_username: Option<String>,
    download_password: Option<String>,
    download_url: Option<String>,
    enable_download: Option<bool>,
    download_tool_path: Option<String>
) {
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
    let password = password.unwrap_or(ask_password("Please input account password"));
    let password = encode_password(&password);
    log::info!("Initializing database on url: {}, schema: {}", url, schema);
    // Initialize the database
    let data = core::utils::db::init::init(url.as_str(), schema.as_str(), username.as_str(), password.as_str());
    if data.is_err() {
        log::error!("Error with database: {:?}", data.err());
        return;
    }
    log::info!("Database initialized");
    // check downloader.
    let enable_download = enable_download.unwrap_or(ask_yes("Enable downloader?"));
    let mut download_username = download_username;
    let mut download_password = download_password;
    let mut download_url = download_url;
    let mut download_tool_path = download_tool_path;
    if enable_download {
        download_tool_path = if download_tool_path.is_none() {
            let download_tool_type = ask_select("Please choose download connecter(tools) type", vec!["qbittorrent", "local_file"]);
            if download_tool_type == "qbittorrent" {
                // todo: download tool and auto add path.
                Some("".to_string())
            } else {
                Some(ask_input_without_hint("Please input download tool path(mac: .dylib, linux: .so, win: .dll)"))
            }   
        } else {
            download_tool_path
        };
        // check sign.
        let lib_file = shellexpand::tilde( &download_tool_path.clone().unwrap()).to_string();
        if !path::Path::new(&lib_file).exists() {
            log::error!("Download tool path does not exist: {}", download_tool_path.clone().unwrap());
            return;
        }
        // check libloading sign.
        unsafe {
            let lib = libloading::Library::new(lib_file);
            let mut is_warn = false;
            if lib.is_err() {
                log::error!("Failed to load download tool library.");
                is_warn = true;
            } else {
                let lib = lib.unwrap();
                if lib.get::<StartFunc>(b"start").is_err() {
                    log::error!("Cannot find significant sign: start");
                    is_warn = true;
                }
                if lib.get::<DownloadFunc>(b"download_by_link").is_err() {
                    log::error!("Cannot find significant sign: download_by_link");
                    is_warn = true;
                }
            }
            if is_warn {
                log::error!("Notice This library may not be able to use. if you don't know why show this, please use default way download. or make issue on repo.");
            } else {
                log::info!("Download tool library check successful.");
            }
        }
        download_username = Some(download_username.unwrap_or(ask_input("Please input download username", "admin")));
        download_password = Some(download_password.unwrap_or(ask_password("Please input your downloader password")));
        download_url = Some(download_url.unwrap_or(ask_input("Please input download url", "http://localhost:8080")));
    }
    let download_tool_path = download_tool_path.unwrap();
    let config_path = config.unwrap_or("~/.bgmdl/config.json".to_string()); //TODO: windows will not use this path.
    let config_path = path::PathBuf::from(shellexpand::tilde(&config_path).to_string());
    fs::create_dir_all(config_path.clone().parent().unwrap()).unwrap();
    let config = Json!{
        "database": {
            "url": url,
            "schema": schema,
        },
        "port": port.unwrap_or(1824),
        "download": {
            "username": download_username.unwrap_or("".to_string()),
            "password": download_password.unwrap_or("".to_string()),
            "url": download_url.unwrap_or("".to_string()),
            "tool_path": download_tool_path,
            "enable": enable_download,
        }
    };
    let data = fs::write(config_path, &config);
    if data.is_err() {
        log::error!("Error with config file: {:?}", data.err());
        return;
    }
    log::info!("Config file initialized");
    log::info!("Initialization completed");
}