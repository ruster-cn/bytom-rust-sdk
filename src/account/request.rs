use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountRequestParam{
    pub root_xpubs: Vec<String>,
    pub alias: String,
    pub quorum: u8,
}

