use sea_orm::DbErr;

#[derive(Debug)]
pub enum CoreError {
    DbError(DbErr),
    StdError(Box<dyn std::error::Error>),
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
