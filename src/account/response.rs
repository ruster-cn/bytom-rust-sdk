use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Receiver {
    pub control_program: String,
    pub address:String,
}

#[derive(Debug,Default,Deserialize,Serialize)]
pub struct AddressResp{
    pub account_alias: String,
    pub account_id: String,
    pub address: String,
    pub control_program: String,
    pub change: bool,
    pub key_index: u64,
}

#[derive(Debug,Default,Deserialize,Serialize)]
pub struct ValidateAddressResp{
    pub valid: bool,
    pub is_local: bool,
}

#[derive(Debug,Default,Deserialize,Serialize)]
pub struct MiningAddressResp{
    pub mining_address: String
}

#[derive(Debug,Default,Deserialize,Serialize)]
pub struct AccountPubkey{
    pub root_xpub: String,
    pub pubkey_infos: Vec<PubKeyInfo>
}

#[derive(Debug,Default,Deserialize,Serialize)]
pub struct PubKeyInfo{
    pub pubkey: String,
    pub derivation_path: Vec<String>
}

#[derive(Debug,Default,Deserialize,Serialize)]
pub struct AccountBalance{
    pub account_id: String,
    pub account_alias: String,
    pub asset_alias: String,
    pub asset_id: String,
    pub amount: u64,
    pub asset_definition: HashMap<String,String>,
}

#[derive(Debug,Default,Deserialize,Serialize)]
pub struct GetWorkJSONResp{
    pub block_header: BlockHeaderJSON,
    pub seed: String
}

#[derive(Debug,Default,Deserialize,Serialize)]
pub struct BlockHeaderJSON{
    pub version:u64,
    pub height: u64,
    pub previous_block_hash: String,
    pub timestamp:u64,
    pub nonce:u64,
    pub bits:u64,
    pub block_commitment: BlockCommitment,
}

#[derive(Debug,Default,Deserialize,Serialize)]
pub struct BlockCommitment{
    pub transaction_merkle_root: String,
    pub transaction_status_hash: String,
}



