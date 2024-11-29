use core::declare::error::CoreError;
use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::derive::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum HttpError {
    #[display("Core Error")]
    CoreError,
    #[display("IO Error")]
    IOError,
}

impl error::ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(Json!{
                "error": self.to_string()
            })
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            HttpError::CoreError | HttpError::IOError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<CoreError> for HttpError {
    fn from(_error: CoreError) -> Self {
        HttpError::CoreError
    }
}

pub type ResultHandler<T> = Result<T, HttpError>;
pub mod user;
pub mod task;