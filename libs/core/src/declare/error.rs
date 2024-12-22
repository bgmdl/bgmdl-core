use derive_more::derive::Display;
use sea_orm::DbErr;

#[derive(Debug, Display)]
pub enum CoreError {
    #[display("(CoreError) Db Error")]
    DbError(DbErr),
    #[display("(CoreError) Std Error")]
    StdError(Box<dyn std::error::Error>),
    #[display("(CoreError) Error")]
    StringError(String),
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
