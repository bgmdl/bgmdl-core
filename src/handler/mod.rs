use std::error::Error;

pub type ResultHandler<T> = Result<T, Box<dyn Error>>;
pub mod user;
pub mod task;