pub mod request;
pub mod response;

use serde::export::fmt::Error;
use serde::export::Formatter;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct AnnotatedAccount {
    //id account id
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    //alias name of account
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub alias: String,
    //xpubs pubkey array
    #[serde(default)]
    pub xpubs: Vec<String>,
    //quorom threshold of keys that must sign a transaction to spend asset units controlled by the account
    #[serde(default)]
    pub quorum: u8,
    //key_index key index of account
    #[serde(default)]
    pub key_index: u64,
    #[serde(default)]
    pub derive_rule: u8,
}

impl fmt::Display for AnnotatedAccount {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let s = serde_json::to_string(self);
        match s {
            Ok(s) => write!(f, "{}", s),
            Err(err) => write!(f, "{}", err.to_string()),
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Address {
    //account_alias alis of account
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub account_alias: String,
    //account_id id of account
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub account_id: String,
    //address address of account
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub address: String,
    //program control program of account
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub control_program: String,
    //change whethe the account address is change
    #[serde(default)]
    pub change: bool,
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let s = serde_json::to_string(self);
        match s {
            Ok(s) => write!(f, "{}", s),
            Err(err) => write!(f, "{}", err.to_string()),
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AddressVec(Vec<Address>);

impl fmt::Display for AddressVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let s = serde_json::to_string(self);
        match s {
            Ok(s) => write!(f, "{}", s),
            Err(err) => write!(f, "{}", err.to_string()),
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Image {
    pub slices: Vec<ImageSlice>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ImageSlice {
    pub account: Account,
    pub contract_index: u64,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Account {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub account_type: String,
    pub xpubs: Vec<String>,
    pub quorum: u8,
    pub key_index: u64,
    pub derive_rule: u8,
    pub id: String,
    pub alias: String,
}
