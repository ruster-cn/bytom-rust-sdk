use serde::export::fmt::Error;
use serde::export::Formatter;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct Key {
    //alias name of the key
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub alias: String,
    //password of the key
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub password: String,
    //language mnemonic language of the key
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub language: String,
    //mnemonic of the key,create key by specified mnimonic
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub mnemonic: String,
    //xpub root pubkey of the key
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub xpub: String,
    //file path to the file of key
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub file: String,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let s = serde_json::to_string(&self);
        match s {
            Ok(s) => write!(f, "{}", s),
            Err(err) => write!(f, "{}", err.to_string()),
        }
    }
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct CheckPasswordResp {
    #[serde(default)]
    pub check_result: bool,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct ResetPasswordResp {
    #[serde(default)]
    pub changed: bool,
}

#[cfg(test)]
mod tests {
    use crate::key::Key;
    use crate::key::XPub;
    #[test]
    fn test_display_key() {
        let test = Key {
            alias: "test".to_string(),
            password: "test".to_string(),
            language: "en".to_string(),
            mnemonic: "test".to_string(),
            xpub: "".to_string(),
            file: "".to_string(),
        };
        println!("{}", test);
    }
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct Token {
    pub id: String,
    pub token: String,
    pub token_type: String,
    pub create_at: u64,
}
