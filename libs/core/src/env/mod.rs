use std::sync::{Arc, Mutex};

use download_link::Callback;
use lazy_static::lazy_static;

use crate::utils::pluginload::DownloadHandler;

lazy_static! {
    pub static ref DOWNLOAD_PATH: Mutex<String> = Mutex::new(String::from(""));
    pub static ref DOWNLOAD_HANDLER: Mutex<DownloadHandler> = Mutex::new({
        log::info!(
            "init download handler from {}",
            DOWNLOAD_PATH.lock().unwrap().as_str()
        );
        DownloadHandler::new(DOWNLOAD_PATH.lock().unwrap().as_str())
    });
    pub static ref DOWNLOAD_CALLBACK_FUNC: Mutex<Callback> = Mutex::new(crate::service::default_callback_func);
    pub static ref DOWNLOAD_CALLBACK_FUNC_REF: Arc<Mutex<&'static Callback>> =
        Arc::new(Mutex::new(&(crate::service::default_callback_func as Callback)));
}
