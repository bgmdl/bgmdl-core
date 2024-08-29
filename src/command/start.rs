//! router: start
//! description: Start the server
//! --port -p <port>, Port to listen

pub fn run(port: Option<u32>) {
    println!("Start the server {}", port.unwrap_or(8060));
    if let Some(array) = array {
        println!("Array: {:?}", array);
    }
}