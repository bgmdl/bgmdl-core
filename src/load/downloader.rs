// use std::{os::raw::{c_char, c_void}, path::Path};

// use download_link::DownloadData;
// use download_link::DownloadTools;
// use libloading::{Library, Symbol};

// type DownloadToolsLoad = unsafe fn() -> *mut dyn DownloadTools;

// pub fn get_download_tool(path: &str) -> Box<dyn DownloadTools> {
//     let lib_path = Path::new(path);
//     let lib =  unsafe{ Library::new(lib_path).unwrap() };
//     unsafe {
//         let plugin_create: Symbol<DownloadToolsLoad> = lib.get(b"apply").unwrap();
//         Box::from_raw(plugin_create())
//     }
// }

// type CallBack = fn(*mut c_void, DownloadData);

// pub type StartFunc = unsafe fn(link: *const c_char, username: *const c_char, password: *const c_char) -> i32;
// pub type DownloadFunc = unsafe fn(url: *const c_char, save_path: *const c_char, rename: *const c_char, callback: CallBack) -> i32;

// #[cfg(test)]
// mod tests {
//     use std::{ffi::CString, thread::sleep};

//     use super::*;

//     #[test]
//     fn test_load_qt() {
//         dbg!("test");
//         println!("test");
//         let lib_path = Path::new("target/debug/libqb.dylib");
//         let lib =  unsafe{ Library::new(lib_path).unwrap() };
//         let link = CString::new("").unwrap();
//         let username = CString::new("").unwrap();
//         let password = CString::new("").unwrap();
//         unsafe {
//             let startfunc = lib.get::<StartFunc>(b"start").unwrap();
//             // dbg!(startfunc);
//             let result = startfunc(link.as_ptr(), username.as_ptr(), password.as_ptr());
//             dbg!(result);

//         }

//         unsafe {
//             let downloadfunc = lib.get::<DownloadFunc>(b"download_by_link").unwrap();
//             let link = CString::new("magnet:?xt=urn:btih:5270c0ab0c34a956ccac75d7d884dcda5f8aa8bc&dn=%5BANi%5D%20Bleach%20%2F%20%20BLEACH%20%E6%AD%BB%E7%A5%9E%20%E5%8D%83%E5%B9%B4%E8%A1%80%E6%88%B0%E7%AF%87-%E7%9B%B8%E5%89%8B%E8%AD%9A-%20-%2030%20%5B1080P%5D%5BBaha%5D%5BWEB-DL%5D%5BAAC%20AVC%5D%5BCHT%5D%5BMP4%5D&tr=http%3A%2F%2Fnyaa.tracker.wf%3A7777%2Fannounce&tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.torrent.eu.org%3A451%2Fannounce").unwrap();
//             let save_path = CString::new("/tmp").unwrap();
//             let rename = CString::new("test").unwrap();
//             sleep(std::time::Duration::from_secs(1));
//             downloadfunc(link.as_ptr(), save_path.as_ptr(), rename.as_ptr(), |_, data| {
//                 println!("test");
//                 dbg!(data);
//             });
//         }
//         loop {
//         };
//     }
// }