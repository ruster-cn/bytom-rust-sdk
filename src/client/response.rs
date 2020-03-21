use serde::Deserialize;
use serde::Serialize;

use serde::export::fmt::Error;
use serde::export::Formatter;
use std::fmt;

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Response<T>
where
    T: Serialize,
{
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub status: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub code: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub msg: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub detail: String,
    #[serde(default)]
    pub temporary: bool,
    #[serde(default)]
    pub data: T,
}

impl<T> fmt::Display for Response<T>
where
    T: Serialize,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let s = serde_json::to_string(&self);
        match s {
            Ok(s) => write!(f, "{}", s),
            Err(err) => write!(f, "{}", err.to_string()),
        }
    }
}

impl<T> Response<T>
where
    T: Serialize,
{
    pub fn is_success(&self) -> bool {
        if self.status == "success" {
            true
        } else {
            false
        }
    }
}
