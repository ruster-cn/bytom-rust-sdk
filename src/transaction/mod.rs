use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

pub mod request;
pub mod response;

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct AssetImage {
    pub assets: Vec<Asset>,
}

//todo definition supprot dynamic typing
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct Asset {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub account_type: String,
    pub xpubs: Vec<String>,
    pub quorum: u8,
    pub key_index: u64,
    pub derive_rule: u8,
    pub id: String,
    pub alias: String,
    pub vm_version: String,
    pub issue_program: String,
    pub raw_definition_byte: String,
    pub definition: HashMap<String, String>,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct AnnotatedTx {
    pub tx_id: String,
    pub block_time: String,
    pub block_hash: String,
    pub block_height: u64,
    pub block_index: u32,
    pub block_transactions_count: u32,
    pub inputs: Vec<AnnotatedInput>,
    pub outputs: Vec<AnnotatedOutput>,
    pub status_fail: bool,
    pub size: u64,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct AnnotatedInput {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub input_type: String,
    pub asset_id: String,
    pub asset_alias: String,
    pub asset_definition: String,
    pub amount: u64,
    pub issuance_program: String,
    pub control_program: String,
    pub address: String,
    pub spent_output_id: String,
    pub account_id: String,
    pub account_alias: String,
    pub arbitrary: String,
    pub input_id: String,
    pub witness_arguments: Vec<String>,
    pub sign_data: String,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct AnnotatedOutput {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub output_type: String,
    pub id: String,
    pub transaction_id: String,
    pub position: u8,
    pub asset_id: String,
    pub asset_alias: String,
    pub asset_definition: String,
    pub amount: u64,
    pub account_id: String,
    pub account_alias: String,
    pub control_program: String,
    pub address: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TransactionFeed {
    pub id: String,
    pub alias: String,
    pub filter: String,
    pub param: TransactionFeedParam,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TransactionFeedParam {
    pub assetid: String,
    pub lowerlimit: u64,
    pub uppperlimit: u64,
    pub transtype: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct BlockTx {
    pub id: String,
    pub version: u64,
    pub size: u64,
    pub time_range: u64,
    pub inputs: Vec<AnnotatedInput>,
    pub outputs: Vec<AnnotatedOutput>,
    pub status_fail: bool,
    pub mux_id: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RawTx {
    pub tx_id: String,
    pub version: u64,
    pub size: u64,
    pub time_range: u64,
    pub inputs: Vec<AnnotatedInput>,
    pub outputs: Vec<AnnotatedOutput>,
    pub fee: u64,
}
