use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct CheckLoginProps {
    pub username: String,
    pub password: String,
}