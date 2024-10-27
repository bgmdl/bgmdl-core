use std::{ffi::CString, path::Path};
use libloading::{Library, Symbol};
use download_link::{Callback, DownloadFunc, StartFunc};

pub struct DownloadHandler {
    lib: Library,
}

impl DownloadHandler {
    pub fn new(path: &str) -> Self {
        let lib_path = Path::new(path);
        let lib = unsafe { Library::new(lib_path).unwrap() };
        DownloadHandler {
            lib,
        }
    }

    pub fn start(&self, link: &str, username: &str, password: &str) -> i32 {
        // Call the function if needed
        let link = CString::new(link).unwrap();
        let username = CString::new(username).unwrap();
        let password = CString::new(password).unwrap();
        let symbol: Symbol<StartFunc> = unsafe { self.lib.get(b"start").unwrap() };
        unsafe {
            (symbol)(link.as_ptr(), username.as_ptr(), password.as_ptr())
        }
    }

    pub fn download_by_link(&self, url: &str, savepath: &str, rename: &str, callback: Callback) -> i32 {
        // Call the function if needed
        let url = CString::new(url).unwrap();
        let savepath = CString::new(savepath).unwrap();
        let rename = CString::new(rename).unwrap();
        let symbol: Symbol<DownloadFunc> = unsafe { self.lib.get(b"download_by_link").unwrap() };
        unsafe {
            (symbol)(url.as_ptr(), savepath.as_ptr(), rename.as_ptr(), callback)
        }
    }
}