use core::utils::db::conn::get_connect;
use std::os::raw::c_void;

use download_link::DownloadData;

pub extern "C" fn callback(_: *mut c_void, data: DownloadData) {
    log::trace!("Download progress: {:?}", &data);
    let db = async_run! {
        get_connect(get_env!(dblink).as_str(), get_env!(dbschema).as_str()).await.ok().unwrap()
    };
    async_run! {
        let _ = core::model::task::add_task_progress(data.taskid, data.progress, data.speed, &db).await;
    };
}