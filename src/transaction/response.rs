use crate::transaction::BlockTx;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CoinbaseArbitrary {
    pub arbitrary: String,
}

//todo  The field definition supports dynamic typing.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AnnotateAsset {
    pub id: String,
    pub alias: String,
    pub vm_version: u64,
    pub issue_program: String,
    pub raw_definition_byte: String,
    pub definition: HashMap<String, String>,
    pub limit_height: u64,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub annotate_type: String,
    pub xpubs: Vec<String>,
    pub quorum: u8,
    pub key_index: u64,
    pub derive_rule: u8,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AnnotatedUTXO {
    pub account_alias: String,
    pub id: String,
    pub asset_id: String,
    pub asset_alias: String,
    pub amount: u64,
    pub account_id: String,
    pub address: String,
    pub control_program_index: u64,
    pub program: String,
    pub source_id: String,
    pub source_pos: u64,
    pub valid_height: u64,
    pub change: bool,
    pub derive_rule: u8,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SignMsgResp {
    pub signature: String,
    pub derived_xpub: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DecodeProgResp {
    pub instructions: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Template {
    pub raw_transaction: String,
    pub fee: u64,
    pub allow_additional_actions: bool,
    pub signing_instructions: Vec<SigningInstruction>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SigningInstruction {
    pub position: u32,
    pub witness_components: Vec<WitnessComponent>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct WitnessComponent {
    pub quorum: u8,
    pub keys: Vec<String>,
    pub signatures: Vec<String>,
    pub program: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SignTemplateResp {
    pub transaction: Template,
    pub sign_complete: bool,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SignTemplatesResp {
    pub transaction: Vec<Template>,
    pub sign_complete: bool,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SubmitTxResp {
    pub tx_id: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SubmitTxsResp {
    pub tx_id: Vec<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct EstimateTxGasInfo {
    pub total_neu: u64,
    pub flexible_neu: u64,
    pub storage_neu: u64,
    pub vm_neu: u64,
    pub chain_tx_neu: u64,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct UnconfirmedTxsResp {
    pub total: u64,
    pub tx_ids: Vec<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GetBlockResp {
    pub hash: String,
    pub size: u64,
    pub version: u64,
    pub height: u64,
    pub previous_block_hash: String,
    pub timestamp: u64,
    pub nonce: u64,
    pub bits: u64,
    pub difficulty: String,
    pub transaction_merkle_root: String,
    pub transaction_status_hash: String,
    pub transactions: Vec<BlockTx>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GetBlockHeaderResp {
    pub block_header: String,
    pub reward: u64,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GetDifficultyResp {
    pub hash: String,
    pub height: u64,
    pub bits: u64,
    pub difficulty: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GetHashRateResp {
    pub hash: String,
    pub height: u64,
    pub hash_rate: u64,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct VerifyMsgResp {
    pub result: bool,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ComileResp {
    pub name: String,
    pub source: String,
    pub program: String,
    pub parmas: Vec<ComileParams>,
    pub value: String,
    pub clause_info: Vec<Clause>,
    pub opcode: String,
    pub error: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ComileParams {
    pub name: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub type_desc: String,
    pub inferred_type: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Clause {
    pub name: String,
    pub params: Vec<ComileParams>,
    pub blockheight: Vec<String>,
    pub hash_calls: Vec<HashCall>,
    pub values: Vec<ValueInfo>,
    pub contracts: Vec<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct HashCall {
    pub hash_type: String,
    pub arg: String,
    pub arg_type: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ValueInfo {
    pub name: String,
    pub program: String,
    pub asset: String,
    pub amount: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GetWorkResp {
    pub block_header: String,
    pub seed: String,
}
