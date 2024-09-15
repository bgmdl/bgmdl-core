//! router: init
//! description: Initialize the basic data.
//! --url -u <url>, database url
//! --schema -s <schema>, schema name
//! --username -U <username>, username
//! --password -P <password>, password
//! --port -p <port>, Init the server port

use serde::de;

pub fn ask_input(hints: &str, default: &str) -> String {
    let result = inquire::Text::new(hints).with_default(default).prompt();
    match result {
        Ok(result) => result,
        Err(_) => {
            println!("Error with input, try again later.");
            ask_input(hints, default)
        },
    }
}

pub fn ask_password(hints: &str) -> String {
    let result = inquire::Password::new(hints).prompt();
    match result {
        Ok(result) => result,
        Err(_) => {
            println!("Error with input, try again later.");
            ask_password(hints)
        },
    }
}

pub fn run(url: Option<String>, schema: Option<String>, username: Option<String>, password: Option<String>, port: Option<u32>) {
    let url = url.unwrap_or(ask_input("Please input database url", "postgres://localhost:5432"));
    let schema = schema.unwrap_or(ask_input("Please input schema name", "public"));
    let username = username.unwrap_or(ask_input("Please input username", "admin"));
    let password = password.unwrap_or(ask_password("Please input password"));
    log::info!("Initializing database on url: {}, schema: {}", url, schema);
    // Initialize the database
    let data = core::db::init::init(url.as_str(), schema.as_str());
    if data.is_err() {
        log::error!("Error with database: {:?}", data.err());
        return;
    }
}