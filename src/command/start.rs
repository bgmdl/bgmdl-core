//! router: start
//! description: Start the server
//! --config -c <config>, config path (optional\, ~/.bgmdl/config.json)
//! --port -p <port>, server port (optional\, default: read by config or 1824)

use crate::{handle, DEFAULT_PORT};

pub fn run(config: Option<String>, port: Option<u32>) {
    let _ = config;
    let port = port.unwrap_or(DEFAULT_PORT as u32) as u16;
    log::info!("Starting server on port: {}", port);
    // Start the core server 
    // Start the server
    let _ = handle::main(port);
}
