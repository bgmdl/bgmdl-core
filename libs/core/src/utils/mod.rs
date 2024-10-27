pub mod error;
pub mod parsetitle;
pub mod regex;
pub mod data;
pub mod pluginload;


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