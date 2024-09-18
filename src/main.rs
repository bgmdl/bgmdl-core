use std::env;

use env_logger::Builder;
use log::LevelFilter;
use macro_lib::generate_commands;

include!("./require.rs");
include!("./default.rs");

pub static mut DBLINK: state::InitCell<String> = state::InitCell::new();
pub static mut DBSCHEMA: state::InitCell<String> = state::InitCell::new();

generate_commands!();

fn main() {
    if env::var("LOG_LEVEL").is_err() {
        env::set_var("LOG_LEVEL", "info"); // set default log level
    }
    Builder::new()
        .filter_module("sqlx::query", LevelFilter::Warn)
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