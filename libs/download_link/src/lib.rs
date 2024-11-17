use std::ffi::CString;
use std::os::raw::{c_char, c_void};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct DownloadData {
    pub name: *const c_char,
    pub progress: f64,
    pub taskid: i32,
    pub speed: i64,
    pub eta: i64,
}

use log::{LevelFilter, Log, Metadata, Record};

#[repr(C)]
pub struct LogParam {
    pub enabled: extern "C" fn(&Metadata) -> bool,
    pub log: extern "C" fn(&Record),
    pub flush: extern "C" fn(),
    pub level: LevelFilter,
}

struct DLog;

static mut PARAM: Option<LogParam> = None;

pub fn init(param: LogParam) {
    let level = param.level;
    unsafe {
        if PARAM.is_some() {
            eprint!("log should only init once");
            return;
        }
        PARAM.replace(param);
    }
    if let Err(err) = log::set_logger(&LOGGER).map(|_| log::set_max_level(level)) {
        eprint!("set logger failed:{}", err);
    }
}

fn param() -> &'static LogParam {
    unsafe { PARAM.as_ref().unwrap() }
}

impl Log for DLog {
    fn enabled(&self, metadata: &Metadata) -> bool {
        (param().enabled)(metadata)
    }

    fn log(&self, record: &Record) {
        (param().log)(record)
    }

    fn flush(&self) {
        (param().flush)()
    }
}

static LOGGER: DLog = DLog;

#[no_mangle]
extern "C" fn enabled(meta: &Metadata) -> bool {
    log::logger().enabled(meta)
}

#[no_mangle]
extern "C" fn log(record: &Record) {
    log::logger().log(record)
}

#[no_mangle]
extern "C" fn flush() {
    log::logger().flush()
}

pub fn log_param() -> LogParam {
    LogParam {
        enabled,
        log,
        flush,
        level: log::max_level(),
    }
}

unsafe impl Send for DownloadData {}

pub type Callback = extern "C" fn(*mut c_void, data: DownloadData);
pub type StartFunc =
    unsafe fn(link: *const c_char, username: *const c_char, password: *const c_char) -> i32;
pub type DownloadFunc = unsafe fn(
    taskid: i32,
    url: *const c_char,
    save_path: *const c_char,
    rename: *const c_char,
    callback: Callback,
) -> i32;

impl DownloadData {
    pub fn new(name: &str, progress: f64, speed: i64, eta: i64, taskid: i32) -> Self {
        DownloadData {
            name: CString::new(name).unwrap().into_raw(),
            progress,
            speed,
            eta,
            taskid,
        }
    }
}
