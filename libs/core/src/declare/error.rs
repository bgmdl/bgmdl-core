use derive_more::derive::Display;
use sea_orm::DbErr;

#[derive(Debug, Display)]
pub enum CoreError {
    #[display("Db Error")]
    DbError(DbErr),
    #[display("Std Error")]
    StdError(Box<dyn std::error::Error>),
    #[display("Error: {}", _0)]
    StringError(String),
    #[display("Actix Error")]
    ReqwestError(reqwest::Error),
    #[display("Json Parse Error")]
    ParseError(serde_json::Error),
    #[display("Not Found")]
    NotFound,
}

impl From<reqwest::Error> for CoreError {
    fn from(err: reqwest::Error) -> Self {
        CoreError::ReqwestError(err)
    }
}

impl From<serde_json::Error> for CoreError {
    fn from(err: serde_json::Error) -> Self {
        CoreError::ParseError(err)
    }
}

impl From<String> for CoreError {
    fn from(err: String) -> Self {
        CoreError::StringError(err)
    }
}

impl From<DbErr> for CoreError {
    fn from(err: DbErr) -> Self {
        CoreError::DbError(err)
    }
}

impl From<Box<dyn std::error::Error>> for CoreError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        CoreError::StdError(err)
    }
}
