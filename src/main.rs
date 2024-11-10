use std::{env, sync::Mutex};

use env_logger::Builder;
use lazy_static::lazy_static;
use log::LevelFilter;
use macro_lib::generate_commands;

include!("./require.rs");
include!("./default.rs");
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

lazy_static! {
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

generate_commands!();

fn main() {
    if env::var("LOG_LEVEL").is_err() {
        env::set_var("LOG_LEVEL", "info"); // set default log level
    }
    Builder::new()
        .filter_module("sqlx::query", LevelFilter::Warn)
        .filter_module("actix_server::server", LevelFilter::Warn)
        .parse_env("LOG_LEVEL")
        .init();
    execute_command();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
