//! router: changepwd
//! description: Change the password
//! --config -c <config>, config path (optional\, ~/.bgmdl/config.json)
//! args: [new_password](change_to_new_password)

use core::utils::db::conn::get_connect;
use core::model;

use crate::utils::{config_load::env_load, inquire::*};

pub fn run(new_password: Option<String>, config: Option<String>) {
    // load config file.
    env_load(&config.unwrap_or("~/.bgmdl/config.json".to_string()));
    let new_password = new_password.unwrap_or(ask_password("Please input new password"));
    let db = get_connect(get_env!(dblink).as_str(), get_env!(dbschema).as_str());
    if db.is_err() {
        log::error!("Get database connection failed: {:?}", db.err().unwrap());
        return;
    }
    let db = db.unwrap();
    // change password.
    let _ = model::user::change_password(&new_password, &db);
}