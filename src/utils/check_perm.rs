use core::{model::user::check_user, utils::db::conn::get_connect};

use actix_web::{cookie::Cookie, HttpRequest};

pub async fn check_user_permission(req: &HttpRequest, _perm: &str) -> bool {
    check_user(
        req.cookie("_uid").unwrap_or(Cookie::new("_uid", "-1")).value(),
        req.cookie("_pwd").unwrap_or(Cookie::new("_pwd", "-1")).value(),
        &get_connect(get_env!(dblink).as_str(), get_env!(dbschema).as_str()).await.ok().unwrap(),
    ).await.ok().unwrap_or(false)
}