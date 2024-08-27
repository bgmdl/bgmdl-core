use std::path::Path;

use download_link::{DownloadTools};
use libloading::{Library, Symbol};

type DownloadToolsLoad = unsafe fn() -> *mut dyn DownloadTools;

fn main() {
    let lib_path = Path::new("./target/debug/libqb.dylib");  // 根据你的系统选择合适的扩展名
    let lib =  unsafe{ Library::new(lib_path).unwrap() };
    let mut plugin = unsafe {
        // 加载符号
        let plugin_create: Symbol<DownloadToolsLoad> = lib.get(b"apply").unwrap();
        Box::from_raw(plugin_create())
    };
    println!("start");
    let _ = plugin.login("", "", "");
    let _ = plugin.progress_update_run(Box::new(move |data| {
        // dbg!(data);
        println!("progress: {:?}", data);
    }));
    println!("ee");
    let _ = plugin.download_by_link("magnet:?xt=urn:btih:e2db2dfe5835801ccf54c2a32274d5d0648b2430&dn=%5BToonsHub%5D%20MAYONAKA%20PUNCH%20S01E08%201080p%20CR%20WEB-DL%20AAC2.0%20H.264%20%28Multi-Subs%29&tr=http%3A%2F%2Fnyaa.tracker.wf%3A7777%2Fannounce&tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.torrent.eu.org%3A451%2Fannounce", "default", "test1");
    println!("start");
    // sleep 10 second
    std::thread::sleep(std::time::Duration::from_secs(10));
    println!("test");
    std::thread::sleep(std::time::Duration::from_secs(2));
    loop {
        
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}