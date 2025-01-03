// use serde_json::ser::Formatter;
pub mod encryption;
pub mod inquire;
pub mod config_load;
pub mod logger;

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
        $crate::RUNENV.lock().unwrap().clone()
    };
    ($field:ident) => {{
        let result = $crate::RUNENV.lock().unwrap().$field.clone();
        result
    }};
    ($field:ident $(. $subfields:ident)*) => {{
        let result = $crate::RUNENV.lock().unwrap().$field$(.$subfields)*.clone();
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

#[macro_export]
macro_rules! async_run_all {
    ($($body:tt)*) => {{
        let bt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
        let result = std::panic::catch_unwind(|| { bt.block_on(async {
            $($body)*
        })});
        if let Err(_) = result {
            return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                $($body)*
            });
        } else {
            return result.unwrap();
        }
    }};
}

pub mod check_perm;
pub mod db;