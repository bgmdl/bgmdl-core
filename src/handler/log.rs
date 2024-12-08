use std::thread;

use actix_web::{rt, web, HttpResponse, Scope};
use macro_lib::perm;
use crate::{utils::check_perm::check_user_permission, LOG_DATA};

use super::ResultHandler;


#[perm("log.view")]
pub async fn get_log(stream: web::Payload) -> ResultHandler<HttpResponse> {
    let (res, mut session, _stream) = actix_ws::handle(&req, stream)?;
    rt::spawn(async move {
        loop {
            thread::sleep(std::time::Duration::from_secs(1));
            let log_data = LOG_DATA.lock().unwrap().clone();
            LOG_DATA.lock().unwrap().clear();
            for log in log_data.iter() {
                let _ = session.text(serde_json::to_string(log).unwrap()).await;
            }
        }
    });

    Ok(res)
}

pub fn service() -> Scope {
    web::scope("/api/get_log").route("", web::get().to(get_log))
}