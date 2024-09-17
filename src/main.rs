use env_logger::Builder;
use log::LevelFilter;
use macro_lib::generate_commands;
mod handler; // handler is the controller of the project (web)

include!("./require.rs");
include!("./default.rs");
generate_commands!();


fn main() {
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