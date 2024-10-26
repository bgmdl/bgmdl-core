use std::ffi::CString;
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct DownloadData {
    pub name: *const c_char,
    pub progress: f64,
    pub speed: i64,
    pub eta: i64,
}

unsafe impl Send for DownloadData {}

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