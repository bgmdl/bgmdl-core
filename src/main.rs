use lazy_static::lazy_static;
use macro_lib::generate_commands;
use service::start_async_task_service;
use std::sync::Mutex;

include!("./require.rs");
include!("./default.rs");
include!("./env.rs");

generate_commands!();

fn main() {
    lazy_static::initialize(&TASK_SENDER);
    log::trace!("Please do not use trace level in production.");
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
