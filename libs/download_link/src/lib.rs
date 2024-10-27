use std::ffi::CString;
use std::os::raw::{c_char, c_void};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct DownloadData {
    pub name: *const c_char,
    pub progress: f64,
    pub speed: i64,
    pub eta: i64,
}

unsafe impl Send for DownloadData {}

pub type Callback = extern "C" fn(*mut c_void, data: DownloadData);
pub type StartFunc = unsafe fn(link: *const c_char, username: *const c_char, password: *const c_char) -> i32;
pub type DownloadFunc = unsafe fn(url: *const c_char, savepath: *const c_char, rename: *const c_char, callback: Callback) -> i32;

impl DownloadData {
    pub fn new(name: &str, progress: f64, speed: i64, eta: i64) -> Self {
        DownloadData {
            name: CString::new(name).unwrap().into_raw(),
            progress,
            speed,
            eta,
        }
    }
}