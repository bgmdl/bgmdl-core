use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Database {
	#[serde(rename = "schema")]
	pub schema: Option<String>,

	#[serde(rename = "url")]
	pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Download {
	#[serde(rename = "enable")]
	pub enable: Option<bool>,

	#[serde(rename = "password")]
	pub password: Option<String>,

	#[serde(rename = "tool_path")]
	pub tool_path: Option<String>,

	#[serde(rename = "url")]
	pub url: Option<String>,

	#[serde(rename = "username")]
	pub username: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigJson {
	#[serde(rename = "database")]
	pub database: Option<Database>,

	#[serde(rename = "download")]
	pub download: Option<Download>,

	#[serde(rename = "port")]
	pub port: Option<u16>,
}
