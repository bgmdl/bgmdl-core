//! router: start
//! description: Start the server
//! --port -p <port>, listen port

use crate::{handle, DEFAULT_PORT};

pub fn run(port: Option<u32>) {
    let port = port.unwrap_or(DEFAULT_PORT as u32) as u16;
    log::info!("Starting server on port: {}", port);
    // Start the server
    let _ = handle::main(port);
}