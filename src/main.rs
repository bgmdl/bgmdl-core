use std::env;

use env_logger::Builder;
use macro_lib::generate_commands;

include!("./require.rs");
include!("./default.rs");
generate_commands!();


fn main() {
    Builder::new().parse_env(&env::var("LOG_LEVEL").unwrap_or(DEFAULT_LOG_LEVEL.to_string())).init();
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