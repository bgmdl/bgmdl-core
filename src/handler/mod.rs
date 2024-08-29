use macro_lib::pub_handlers;
use std::error::Error;

pub type ResultHandler<T> = Result<T, Box<dyn Error>>;

pub_handlers!();