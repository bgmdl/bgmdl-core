#[derive(Debug)]
pub enum GetError {
    ReqwestError(reqwest::Error),
    ParseError(serde_json::Error),
}

impl From<reqwest::Error> for GetError {
    fn from(err: reqwest::Error) -> Self {
        GetError::ReqwestError(err)
    }
}

impl From<serde_json::Error> for GetError {
    fn from(err: serde_json::Error) -> Self {
        GetError::ParseError(err)
    }
}
