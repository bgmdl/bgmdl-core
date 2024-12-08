use lazy_static::lazy_static;
use macro_lib::generate_commands;
use std::sync::Mutex;

include!("./require.rs");
include!("./default.rs");
include!("./env.rs");

generate_commands!();

fn main() {
    /*
    if env::var("LOG_LEVEL").is_err() {
        env::set_var("LOG_LEVEL", "info"); // set default log level
    }
    Builder::new()
        .filter_module("sqlx::query", LevelFilter::Warn)
        .filter_module("actix_server::server", LevelFilter::Warn)
        .parse_env("LOG_LEVEL")
        .init(); */
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
