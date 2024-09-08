use std::path::Path;

use download_link::DownloadTools;
use libloading::{Library, Symbol};

type DownloadToolsLoad = unsafe fn() -> *mut dyn DownloadTools;

pub fn get_download_tool(path: &str) -> Box<dyn DownloadTools> {
    let lib_path = Path::new(path);
    let lib =  unsafe{ Library::new(lib_path).unwrap() };
    unsafe {
        let plugin_create: Symbol<DownloadToolsLoad> = lib.get(b"apply").unwrap();
        Box::from_raw(plugin_create())
    }
}