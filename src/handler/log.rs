use std::sync::Mutex;

use actix_web::{web, HttpRequest, HttpResponse, Scope};
use macro_lib::perm;
use crate::{utils::check_perm::check_user_permission, LOG_WS_COUNT, LOG_WS_POLL};

use super::ResultHandler;


#[perm("log.view")]
pub async fn get_log(req: HttpRequest, stream: web::Payload) -> ResultHandler<HttpResponse> {
    let (res, session, _stream) = actix_ws::handle(&req, stream)?;
    let count = *LOG_WS_COUNT.lock().unwrap();
    *LOG_WS_COUNT.lock().unwrap() = count + 1;
    LOG_WS_POLL.lock().unwrap().push((Mutex::new(session.clone()), count + 1));
    /* rt::spawn(async move {
        loop {
            thread::sleep(std::time::Duration::from_secs(1));
            let log_data = LOG_DATA.lock().unwrap().clone();
            LOG_DATA.lock().unwrap().clear();
            for log in log_data.iter() {
                let _ = session.text(serde_json::to_string(log).unwrap()).await;
            }
        }
    }); */
    Ok(res)
}

pub fn service() -> Scope {
    web::scope("/api/get_log").route("", web::get().to(get_log))
}