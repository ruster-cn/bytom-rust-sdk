use crate::account::response::BlockHeaderJSON;
use crate::transaction::response::Template;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

//todo The field definition supports dynamic typing.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CreateAssetRequestParam {
    pub alias: String,
    pub root_xpubs: Vec<String>,
    pub quorum: u8,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub definition: HashMap<String, String>,
    pub limit_height: u64,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub issuance_program: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ListUnspentOutputsRequestParam {
    pub account_id: String,
    pub account_alias: String,
    pub id: String,
    pub unconfirmed: bool,
    pub smart_contract: bool,
    pub from: u64,
    pub count: u8,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ListTransactionsRequestParam {
    pub id: String,
    pub account_id: String,
    pub detail: String,
    pub unconfirmed: bool,
    pub from: u8,
    pub count: u8,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct BuildTransactionsRequestParam {
    pub base_transaction: String,
    pub ttl: u64,
    pub time_range: u64,
    pub actions: Vec<BuildTransactionsActions>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct BuildTransactionsActions {
    pub account_id: String,
    pub account_alias: String,
    pub asset_id: String,
    pub asset_alias: String,
    pub amount: u64,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub action_type: String,
    pub address: String,
    pub control_program: String,
    pub use_unconfirmed: String,
    pub arbitrary: String,
    pub arguments: Vec<ActionsArgument>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ActionsArgument {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub argument_type: String,
    pub raw_data: RawData,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RawData {
    pub xpub: String,
    pub derivation_path: String,
    pub value: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SignTemplateRequestParam {
    pub password: String,
    pub transaction: Template,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SignTemplatesRequestParam {
    pub password: String,
    pub transaction: Vec<Template>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CompileReq {
    pub contract: String,
    pub args: Vec<CompileReqArgs>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CompileReqArgs {
    pub boolean: bool,
    pub integer: u64,
    pub string: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SubmitWorkJSONReq {
    pub block_header: BlockHeaderJSON,
}
