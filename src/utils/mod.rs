// use serde_json::ser::Formatter;
pub mod encryption;
pub mod inquire;
pub mod config_load;

#[macro_export]
macro_rules! Json {
    () => {
        serde_json::json!({}).to_string()
    };

    ($($json:tt)+) => {
        serde_json::json!({$($json)+}).to_string()
    };
}

#[macro_export]
macro_rules! get_env {
    () => {
        crate::RUNENV.lock().unwrap().clone()
    };
    ($field:ident) => {{
        let result = crate::RUNENV.lock().unwrap().$field.clone();
        result
    }};
    ($field:ident $(. $subfields:ident)*) => {{
        let result = crate::RUNENV.lock().unwrap().$field$(.$subfields)*.clone();
        result
    }};
}

#[macro_export]
macro_rules! async_run {
    ($($body:tt)*) => {{
            let bt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        bt.block_on(async {
            $($body)*
        })
    }};
}

pub mod check_perm;