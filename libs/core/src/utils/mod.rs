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
macro_rules! Json {
    () => {
        serde_json::json!({}).to_string()
    };

    ($($json:tt)+) => {
        serde_json::json!({$($json)+}).to_string()
    };
}

pub mod data;
pub mod db;
pub mod error;
pub mod parsetitle;
pub mod pluginload;
pub mod regex;
