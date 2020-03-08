use serde::Deserialize;
use serde::Serialize;
use crate::account::Image as AccountImage;
use crate::transaction::AssetImage;
use std::collections::HashMap;


#[derive(Debug,Default,Deserialize,Serialize)]
pub struct WalletImage{
    pub account_image: AccountImage,
    pub asset_image: AssetImage,
    pub key_images: KeyImage
}

#[derive(Debug,Default,Deserialize,Serialize)]
pub struct KeyImage{
    pub xkeys: Vec<EncryptedKeyJSON>
}

#[derive(Debug,Default,Deserialize,Serialize)]
pub struct EncryptedKeyJSON{
    pub crypto: CryptoJSON,
    pub id: String,
    #[serde(rename(serialize="type",deserialize="tpe"))]
    pub encrypted_key_json_type: String,
    pub version: u8,
    pub alias: String,
    pub xpub: String,
}

#[derive(Debug,Default,Deserialize,Serialize)]
pub struct CryptoJSON{
    pub cipher: String,
    pub ciphertext: String,
    pub cipherparams: CipherparamsJSON,
    pub kdf:String,
    pub kdfparams:HashMap<String,String>,
    pub mac: String,
}

#[derive(Debug,Default,Deserialize,Serialize)]
pub struct CipherparamsJSON{
    pub iv: String,
}

#[derive(Debug,Default,Deserialize,Serialize)]
pub struct WalletInfo{
    pub best_block_height: u64,
    pub wallet_height: u64,
}