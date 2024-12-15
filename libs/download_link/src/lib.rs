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

use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;


lazy_static! {
    static ref PARAM: Arc<Mutex<Option<LogParam>>> = Arc::new(Mutex::new(None));
}

pub fn init(param: LogParam) {
    let level = param.level;
    let mut param_guard = PARAM.lock().unwrap();
    if param_guard.is_some() {
        eprint!("log should only init once");
        return;
    }
    *param_guard = Some(param);
    drop(param_guard); // 显式释放锁

    if let Err(err) = log::set_logger(&LOGGER).map(|_| log::set_max_level(level)) {
        eprint!("set logger failed:{}", err);
    }
}

impl Log for DLog {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let param = PARAM.lock().unwrap();
        (param.as_ref().unwrap().enabled)(metadata)
    }

    fn log(&self, record: &Record) {
        let param = PARAM.lock().unwrap();
        (param.as_ref().unwrap().log)(record)
    }

    fn flush(&self) {
        let param = PARAM.lock().unwrap();
        (param.as_ref().unwrap().flush)()
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
