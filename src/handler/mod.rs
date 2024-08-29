use macro_lib::pub_files;
use std::error::Error;
pub type ResultHandler<T> = Result<T, Box<dyn Error>>;
pub_files!("handler");