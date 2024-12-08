use utils::logger::LogData;


#[derive(Debug, Clone)]
pub struct DEnv {
    pub enable: bool,
    pub password: String,
    pub tool_path: String,
    pub url: String,
    pub username: String,
}

#[derive(Debug, Clone)]
pub struct Env {
    pub dblink: String,
    pub dbschema: String,
    pub download: DEnv,
    pub port: u16,
}

lazy_static! {
    pub static ref LOG_DATA: Mutex<Vec<LogData>> = Mutex::new(vec![]);
    pub static ref RUNENV: Mutex<Env> = Mutex::new(Env {
        dblink: String::from(""),
        dbschema: String::from(""),
        download: DEnv {
            enable: false,
            password: String::from(""),
            tool_path: String::from(""),
            url: String::from(""),
            username: String::from(""),
        },
        port: DEFAULT_PORT,
    });
}

pub static mut DBLINK: state::InitCell<String> = state::InitCell::new();
pub static mut DBSCHEMA: state::InitCell<String> = state::InitCell::new();
