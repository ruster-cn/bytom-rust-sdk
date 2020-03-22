pub mod response;
use crate::account::request::CreateAccountRequestParam;
use crate::account::response::{
    AccountBalance, AccountPubkey, AddressResp, GetWorkJSONResp, MiningAddressResp, Receiver,
    ValidateAddressResp,
};
use crate::account::AnnotatedAccount;
use crate::client::response::Response;
use crate::error::{BtmError, Error};
use crate::key::{CheckPasswordResp, Key, ResetPasswordResp, Token};
use crate::node::{PeerInfo, VersionInfo};
use crate::transaction::request::{
    BuildTransactionsRequestParam, CompileReq, CreateAssetRequestParam,
    ListTransactionsRequestParam, ListUnspentOutputsRequestParam, SignTemplateRequestParam,
    SignTemplatesRequestParam, SubmitWorkJSONReq,
};
use crate::transaction::response::{
    AnnotateAsset, AnnotatedUTXO, CoinbaseArbitrary, ComileResp, DecodeProgResp, EstimateTxGasInfo,
    GetBlockHeaderResp, GetBlockResp, GetDifficultyResp, GetHashRateResp, GetWorkResp, SignMsgResp,
    SignTemplateResp, SignTemplatesResp, SubmitTxResp, SubmitTxsResp, Template, UnconfirmedTxsResp,
    VerifyMsgResp,
};
use crate::transaction::{AnnotatedTx, BlockTx, RawTx, TransactionFeed};
use crate::wallet::{WalletImage, WalletInfo};
use hyper::body::{Buf, Bytes};
use hyper::client::connect::HttpConnector;
use hyper::{Body, Client, Request, Uri};
use std::collections::HashMap;
use std::str;

pub struct BtmClient {
    host: String,
    scheme: String,
    client: Client<HttpConnector>,
}

impl BtmClient {
    pub fn new(host: String, scheme: String) -> BtmClient {
        let client: Client<HttpConnector> = Client::new();
        BtmClient {
            client,
            host,
            scheme,
        }
    }

    async fn create_key_bytes(
        &self,
        alias: &str,
        password: &str,
        language: &str,
        mnemonic: &str,
    ) -> Result<Bytes, Error> {
        let data = format!(
            r#"{{"alias":"{}","password":"{}","language":"{}","mnemonic":"{}"}}"#,
            alias, password, language, mnemonic
        );
        let response_byte = self.post(String::from("/create-key"), data).await?;
        Ok(response_byte)
    }

    pub async fn create_key(
        &self,
        alias: &str,
        password: &str,
        language: &str,
        mnemonic: &str,
    ) -> Result<Key, Error> {
        let response_byte = self
            .create_key_bytes(alias, password, language, mnemonic)
            .await?;
        println!("{:?}", str::from_utf8(response_byte.bytes()));
        let response: Response<Key> = serde_json::from_slice(response_byte.bytes())?;
        if response.is_success() {
            Ok(response.data)
        } else {
            Err(Error::from(BtmError::new(response)))
        }
    }

    async fn list_keys_bytes(&self) -> Result<Bytes, Error> {
        let response_byte = self.post(String::from("/list-keys"), String::new()).await?;
        Ok(response_byte)
    }

    pub async fn list_keys(&self) -> Result<Vec<Key>, Error> {
        let response_byte = self.list_keys_bytes().await?;
        println!("{:?}", str::from_utf8(response_byte.bytes()));
        let response: Response<Vec<Key>> = serde_json::from_slice(response_byte.bytes())?;
        if response.is_success() {
            Ok(response.data)
        } else {
            Err(Error::from(BtmError::new(response)))
        }
    }

    async fn update_key_alias_bytes(&self, xpub: &str, new_alias: &str) -> Result<Bytes, Error> {
        let data = format!(r#"{{"xpub":"{}","new_alias":"{}"}}"#, xpub, new_alias);
        println!("{}", data);
        let response_byte = self.post(String::from("/update-key-alias"), data).await?;
        Ok(response_byte)
    }

    pub async fn update_key_alias(&self, xpub: &str, new_alias: &str) -> Result<bool, Error> {
        let response_byte = self.update_key_alias_bytes(xpub, new_alias).await?;
        println!("{:?}", str::from_utf8(response_byte.bytes()));
        let result: Response<String> = serde_json::from_slice(response_byte.bytes())?;
        Ok(result.is_success())
    }

    async fn delete_key_bytes(&self, xpub: &str, password: &str) -> Result<Bytes, Error> {
        let data = format!(r#"{{"xpub":"{}","password":"{}"}}"#, xpub, password);
        let response_byte = self.post(String::from("/delete-key"), data).await?;
        Ok(response_byte)
    }

    pub async fn delete_key(&self, xpub: &str, password: &str) -> Result<bool, Error> {
        let response_byte = self.delete_key_bytes(xpub, password).await?;
        println!("{:?}", str::from_utf8(response_byte.bytes()));
        let result: Response<String> = serde_json::from_slice(response_byte.bytes())?;
        Ok(result.is_success())
    }

    async fn check_key_password_bytes(&self, xpub: &str, password: &str) -> Result<Bytes, Error> {
        let data = format!(r#"{{"xpub":"{}","password":"{}"}}"#, xpub, password);
        let response_byte = self.post(String::from("/check-key-password"), data).await?;
        Ok(response_byte)
    }

    pub async fn check_key_password(&self, xpub: &str, password: &str) -> Result<bool, Error> {
        let response_byte = self.check_key_password_bytes(xpub, password).await?;
        println!("{:?}", str::from_utf8(response_byte.bytes()));
        let result: Response<CheckPasswordResp> = serde_json::from_slice(response_byte.bytes())?;
        Ok(result.is_success() && result.data.check_result)
    }

    async fn reset_key_password_bytes(
        &self,
        xpub: &str,
        old_password: &str,
        new_password: &str,
    ) -> Result<Bytes, Error> {
        let data = format!(
            r#"{{"xpub":"{}","old_password":"{}","new_password":"{}"}}"#,
            xpub, old_password, new_password
        );
        let response_byte = self.post(String::from("/reset-key-password"), data).await?;
        Ok(response_byte)
    }

    pub async fn reset_key_password(
        &self,
        xpub: &str,
        old_password: &str,
        new_password: &str,
    ) -> Result<bool, Error> {
        let response_byte = self
            .reset_key_password_bytes(xpub, old_password, new_password)
            .await?;
        println!("{:?}", str::from_utf8(response_byte.bytes()));
        let result: Response<ResetPasswordResp> = serde_json::from_slice(response_byte.bytes())?;
        Ok(result.is_success() && result.data.changed)
    }

    async fn create_account_bytes(
        &self,
        create_account_request_param: &CreateAccountRequestParam,
    ) -> Result<Bytes, Error> {
        let data = serde_json::to_string(create_account_request_param)?;
        let response_byte = self.post(String::from("/create-account"), data).await?;
        Ok(response_byte)
    }

    pub async fn create_account(
        &self,
        create_account_request_param: &CreateAccountRequestParam,
    ) -> Result<AnnotatedAccount, Error> {
        let response_byte = self
            .create_account_bytes(create_account_request_param)
            .await?;
        println!("{:?}", str::from_utf8(response_byte.bytes()));
        let result: Response<AnnotatedAccount> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    async fn list_account_bytes(&self, id: &str, alias: &str) -> Result<Bytes, Error> {
        let data = format!(r#"{{"alias":"{}","id":"{}"}}"#, alias, id);
        let response_byte = self.post(String::from("/list-accounts"), data).await?;
        Ok(response_byte)
    }

    pub async fn list_account(
        &self,
        id: &str,
        alias: &str,
    ) -> Result<Vec<AnnotatedAccount>, Error> {
        let response_byte = self.list_account_bytes(id, alias).await?;
        println!("{:?}", str::from_utf8(response_byte.bytes()));
        let result: Response<Vec<AnnotatedAccount>> =
            serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    async fn update_account_alias_bytes(
        &self,
        new_alias: &str,
        old_alias: &str,
        id: &str,
    ) -> Result<Bytes, Error> {
        let data = format!(
            r#"{{"account_id":"{}","account_alias":"{}","new_alias":"{}"}}"#,
            id, old_alias, new_alias
        );
        let response_byte = self
            .post(String::from("/update-account-alias"), data)
            .await?;
        Ok(response_byte)
    }

    pub async fn update_account_alias(
        &self,
        new_alias: &str,
        old_alias: &str,
        id: &str,
    ) -> Result<bool, Error> {
        let response_byte = self
            .update_account_alias_bytes(new_alias, old_alias, id)
            .await?;
        println!("{:?}", str::from_utf8(response_byte.bytes()));
        let result: Response<String> = serde_json::from_slice(response_byte.bytes())?;
        Ok(result.is_success())
    }

    async fn delete_account_bytes(&self, alias: &str, id: &str) -> Result<Bytes, Error> {
        let data = format!(r#"{{"account_id":"{}","account_alias":"{}"}}"#, id, alias);
        let response_byte = self.post(String::from("/delete-account"), data).await?;
        Ok(response_byte)
    }

    pub async fn delete_account(&self, alias: &str, id: &str) -> Result<bool, Error> {
        let response_byte = self.delete_account_bytes(alias, id).await?;
        println!("{:?}", str::from_utf8(response_byte.bytes()));
        let result: Response<String> = serde_json::from_slice(response_byte.bytes())?;
        Ok(result.is_success())
    }

    async fn create_account_receiver_bytes(&self, alias: &str, id: &str) -> Result<Bytes, Error> {
        let data = format!(r#"{{"account_alias":"{}","account_id":"{}"}}"#, alias, id);
        let response_byte = self
            .post(String::from("/create-account-receiver"), data)
            .await?;
        Ok(response_byte)
    }

    pub async fn create_account_receiver(&self, alias: &str, id: &str) -> Result<Receiver, Error> {
        let response_byte = self.create_account_receiver_bytes(alias, id).await?;
        println!("{:?}", str::from_utf8(response_byte.bytes()));
        let result: Response<Receiver> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    async fn list_address_bytes(
        &self,
        alias: &str,
        id: &str,
        from: u32,
        count: u32,
    ) -> Result<Bytes, Error> {
        let data = format!(
            r#"{{"account_alias":"{}","account_id":"{}","from":{},"count":{}}}"#,
            alias, id, from, count
        );
        let response_byte = self.post(String::from("/list-addresses"), data).await?;
        Ok(response_byte)
    }

    pub async fn list_address(
        &self,
        alias: &str,
        id: &str,
        from: u32,
        count: u32,
    ) -> Result<Vec<AddressResp>, Error> {
        let response_byte = self.list_address_bytes(alias, id, from, count).await?;
        println!("{:?}", str::from_utf8(response_byte.bytes()));
        let result: Response<Vec<AddressResp>> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    async fn validate_address_bytes(&self, address: &str) -> Result<Bytes, Error> {
        let data = format!(r#"{{"address":"{}"}}"#, address);
        let response_byte = self.post(String::from("/validate-address"), data).await?;
        Ok(response_byte)
    }

    pub async fn validate_address(&self, address: &str) -> Result<ValidateAddressResp, Error> {
        let response_byte = self.validate_address_bytes(address).await?;
        println!("{:?}", str::from_utf8(response_byte.bytes()));
        let result: Response<ValidateAddressResp> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    async fn get_mining_address_bytes(&self) -> Result<Bytes, Error> {
        let response_byte = self
            .post(String::from("/get-mining-address"), String::new())
            .await?;
        Ok(response_byte)
    }

    pub async fn get_mining_address(&self) -> Result<String, Error> {
        let response_byte = self.get_mining_address_bytes().await?;
        println!("{:?}", str::from_utf8(response_byte.bytes()));
        let result: Response<MiningAddressResp> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data.mining_address)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    async fn set_mining_address_bytes(&self, address: &str) -> Result<Bytes, Error> {
        let data = format!(r#"{{"mining_address":"{}"}}"#, address);
        let response_byte = self.post(String::from("/set-mining-address"), data).await?;
        Ok(response_byte)
    }

    pub async fn set_mining_address(&self, address: &str) -> Result<bool, Error> {
        let response_byte = self.set_mining_address_bytes(address).await?;
        println!("{:?}", str::from_utf8(response_byte.bytes()));
        let result: Response<MiningAddressResp> = serde_json::from_slice(response_byte.bytes())?;
        Ok(result.is_success())
    }

    async fn get_coinbase_arbitrary_bytes(&self) -> Result<Bytes, Error> {
        let response_byte = self
            .post(String::from("/get-coinbase-arbitrary"), String::new())
            .await?;
        Ok(response_byte)
    }

    pub async fn get_coinbase_arbitrary(&self) -> Result<String, Error> {
        let response_byte = self.get_coinbase_arbitrary_bytes().await?;
        println!("{:?}", str::from_utf8(response_byte.bytes()));
        let result: Response<CoinbaseArbitrary> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data.arbitrary)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    async fn set_coinbase_arbitrary_bytes(&self, arbitrary: &str) -> Result<Bytes, Error> {
        let data = format!(r#"{{"arbitrary":"{}"}}"#, arbitrary);
        let response_byte = self
            .post(String::from("/set-coinbase-arbitrary"), data)
            .await?;
        Ok(response_byte)
    }

    pub async fn set_coinbase_arbitrary(&self, arbitrary: &str) -> Result<bool, Error> {
        let response_byte = self.set_coinbase_arbitrary_bytes(arbitrary).await?;
        println!("{:?}", str::from_utf8(response_byte.bytes()));
        let result: Response<CoinbaseArbitrary> = serde_json::from_slice(response_byte.bytes())?;
        Ok(result.is_success())
    }

    async fn list_pubkeys_bytes(
        &self,
        alias: &str,
        id: &str,
        pubkey: &str,
    ) -> Result<Bytes, Error> {
        let data = format!(
            r#"{{"account_alias":"{}","account_id":"{}","public_key":"{}"}}"#,
            alias, id, pubkey
        );
        let response_byte = self.post(String::from("/list-pubkeys"), data).await?;
        Ok(response_byte)
    }

    pub async fn list_pubkeys(
        &self,
        alias: &str,
        id: &str,
        pubkey: &str,
    ) -> Result<AccountPubkey, Error> {
        let response_byte = self.list_pubkeys_bytes(alias, id, pubkey).await?;
        println!("{:?}", str::from_utf8(response_byte.bytes()));
        let result: Response<AccountPubkey> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    async fn create_asset_bytes(&self, param: &CreateAssetRequestParam) -> Result<Bytes, Error> {
        let data = serde_json::to_string(param)?;
        println!("{}", data);
        let response_byte = self.post(String::from("/create-asset"), data).await?;
        Ok(response_byte)
    }

    pub async fn create_asset(
        &self,
        param: &CreateAssetRequestParam,
    ) -> Result<AnnotateAsset, Error> {
        let response_byte = self.create_asset_bytes(param).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<AnnotateAsset> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    async fn get_asset_bytes(&self, id: &str) -> Result<Bytes, Error> {
        let data = format!(r#"{{"id":"{}"}}"#, id);
        let response_byte = self.post(String::from("/get-asset"), data).await?;
        Ok(response_byte)
    }

    pub async fn get_asset(&self, id: &str) -> Result<AnnotateAsset, Error> {
        let response_byte = self.get_asset_bytes(id).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<AnnotateAsset> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    async fn list_assets_bytes(&self, id: &str) -> Result<Bytes, Error> {
        let data = format!(r#"{{"id":"{}"}}"#, id);
        let response_byte = self.post(String::from("/list-assets"), data).await?;
        Ok(response_byte)
    }

    //todo the field of AnnotateAsset definition must support dynamic typing.
    pub async fn list_assets(&self, id: &str) -> Result<Vec<AnnotateAsset>, Error> {
        let response_byte = self.list_assets_bytes(id).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<Vec<AnnotateAsset>> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    async fn update_asset_alias_bytes(&self, id: &str, alias: &str) -> Result<Bytes, Error> {
        let data = format!(r#"{{"id":"{}","alias":"{}"}}"#, id, alias);
        let response_byte = self.post(String::from("/update-asset-alias"), data).await?;
        Ok(response_byte)
    }

    pub async fn update_asset_alias(&self, id: &str, alias: &str) -> Result<bool, Error> {
        let response_byte = self.update_asset_alias_bytes(id, alias).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<String> = serde_json::from_slice(response_byte.bytes())?;
        Ok(result.is_success())
    }

    async fn list_balances_bytes(
        &self,
        account_id: &str,
        account_alias: &str,
    ) -> Result<Bytes, Error> {
        let data = format!(
            r#"{{"account_id":"{}","account_alias":"{}"}}"#,
            account_id, account_alias
        );
        let response_byte = self.post(String::from("/list-balances"), data).await?;
        Ok(response_byte)
    }

    pub async fn list_balances(
        &self,
        account_id: &str,
        account_alias: &str,
    ) -> Result<Vec<AccountBalance>, Error> {
        let response_byte = self.list_balances_bytes(account_id, account_alias).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<Vec<AccountBalance>> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    async fn list_unspent_outputs_bytes(
        &self,
        param: &ListUnspentOutputsRequestParam,
    ) -> Result<Bytes, Error> {
        let data = serde_json::to_string(param)?;
        let response_byte = self
            .post(String::from("/list-unspent-outputs"), data)
            .await?;
        Ok(response_byte)
    }

    pub async fn list_unspent_outputs(
        &self,
        param: &ListUnspentOutputsRequestParam,
    ) -> Result<Vec<AnnotatedUTXO>, Error> {
        let response_byte = self.list_unspent_outputs_bytes(param).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<Vec<AnnotatedUTXO>> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    async fn backup_wallet_bytes(&self) -> Result<Bytes, Error> {
        let response_byte = self
            .post(String::from("/backup-wallet"), String::new())
            .await?;
        Ok(response_byte)
    }

    pub async fn backup_wallet(&self) -> Result<WalletImage, Error> {
        let response_byte = self.backup_wallet_bytes().await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<WalletImage> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn restore_wallet(&self, image: &WalletImage) -> Result<bool, Error> {
        let data = serde_json::to_string(image)?;
        let response_byte = self.post(String::from("/restore-wallet"), data).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<String> = serde_json::from_slice(response_byte.bytes())?;
        Ok(result.is_success())
    }

    pub async fn rescan_wallet(&self) -> Result<bool, Error> {
        let response_byte = self
            .post(String::from("/rescan-wallet"), String::new())
            .await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<String> = serde_json::from_slice(response_byte.bytes())?;
        Ok(result.is_success())
    }

    pub async fn recovery_wallet(&self, xpubs: &Vec<String>) -> Result<bool, Error> {
        let data = serde_json::to_string(xpubs)?;
        let response_byte = self.post(String::from("/recovery-wallet"), data).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<String> = serde_json::from_slice(response_byte.bytes())?;
        Ok(result.is_success())
    }

    pub async fn wallet_info(&self) -> Result<WalletInfo, Error> {
        let response_byte = self
            .post(String::from("/wallet-info"), String::new())
            .await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<WalletInfo> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn sign_message(
        &self,
        address: &str,
        message: &str,
        password: &str,
    ) -> Result<SignMsgResp, Error> {
        let data = format!(
            r#"{{"address":"{}","message":"{}","password":"{}"}}"#,
            address, message, password
        );
        let response_byte = self.post(String::from("/sign-message"), data).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<SignMsgResp> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn decode_message(&self, program: &str) -> Result<DecodeProgResp, Error> {
        let data = format!(r#"{{"program":"{}"}}"#, program);
        let response_byte = self.post(String::from("/decode-program"), data).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<DecodeProgResp> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn get_transaction(&self, tx_id: &str) -> Result<AnnotatedTx, Error> {
        let data = format!(r#"{{"tx_id":"{}"}}"#, tx_id);
        let response_byte = self.post(String::from("/get-transaction"), data).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<AnnotatedTx> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn list_transaction(
        &self,
        param: &ListTransactionsRequestParam,
    ) -> Result<Vec<AnnotatedTx>, Error> {
        let data = serde_json::to_string(param)?;
        let response_byte = self.post(String::from("/list-transactions"), data).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<Vec<AnnotatedTx>> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn build_transaction(
        &self,
        param: &BuildTransactionsRequestParam,
    ) -> Result<Template, Error> {
        let data = serde_json::to_string(param)?;
        let response_byte = self.post(String::from("/build-transactions"), data).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<Template> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn build_chain_transactions(
        &self,
        param: &BuildTransactionsRequestParam,
    ) -> Result<Template, Error> {
        let data = serde_json::to_string(param)?;
        let response_byte = self
            .post(String::from("/build-chain-transactions"), data)
            .await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<Template> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn sign_transaction(
        &self,
        param: &SignTemplateRequestParam,
    ) -> Result<SignTemplateResp, Error> {
        let data = serde_json::to_string(param)?;
        let response_byte = self.post(String::from("/sign-transaction"), data).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<SignTemplateResp> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn sign_transactions(
        &self,
        param: &SignTemplatesRequestParam,
    ) -> Result<SignTemplatesResp, Error> {
        let data = serde_json::to_string(param)?;
        let response_byte = self.post(String::from("/sign-transactions"), data).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<SignTemplatesResp> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn submit_transaction(&self, param: &Template) -> Result<SubmitTxResp, Error> {
        let data = serde_json::to_string(param)?;
        let response_byte = self.post(String::from("/submit-transaction"), data).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<SubmitTxResp> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn submit_transactions(&self, param: &Vec<Template>) -> Result<SubmitTxsResp, Error> {
        let data = serde_json::to_string(param)?;
        let response_byte = self
            .post(String::from("/submit-transactions"), data)
            .await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<SubmitTxsResp> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn estimate_transaction_gas(
        &self,
        param: &Template,
    ) -> Result<EstimateTxGasInfo, Error> {
        let data = serde_json::to_string(param)?;
        let response_byte = self
            .post(String::from("/estimate-transaction-gas"), data)
            .await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<EstimateTxGasInfo> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn create_access_token(&self, id: &str, token_type: &str) -> Result<Token, Error> {
        let data = format!(r#"{{"id":"{}","type":"{}"}}"#, id, token_type);
        let response_byte = self
            .post(String::from("/create-access-token"), data)
            .await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<Token> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn list_access_tokens(&self) -> Result<Vec<Token>, Error> {
        let response_byte = self
            .post(String::from("/list-access-tokens"), String::new())
            .await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<Vec<Token>> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn delete_access_token(&self, id: &str) -> Result<bool, Error> {
        let data = format!(r#"{{"id":"{}"}}"#, id);
        let response_byte = self
            .post(String::from("/delete-access-token"), data)
            .await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<String> = serde_json::from_slice(response_byte.bytes())?;
        Ok(result.is_success())
    }

    pub async fn check_access_token(&self, id: &str, secret: &str) -> Result<bool, Error> {
        let data = format!(r#"{{"id":"{}","secret":"{}"}}"#, id, secret);
        let response_byte = self.post(String::from("/check-access-token"), data).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<String> = serde_json::from_slice(response_byte.bytes())?;
        Ok(result.is_success())
    }

    pub async fn create_transaction_feed(&self, alias: &str, filter: &str) -> Result<bool, Error> {
        let data = format!(r#"{{"alias":"{}","filter":"{}"}}"#, alias, filter);
        let response_byte = self
            .post(String::from("/create-transaction-feed"), data)
            .await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<String> = serde_json::from_slice(response_byte.bytes())?;
        Ok(result.is_success())
    }

    pub async fn get_transaction_feed(&self, alias: &str) -> Result<TransactionFeed, Error> {
        let data = format!(r#"{{"alias":"{}"}}"#, alias);
        let response_byte = self
            .post(String::from("/get-transaction-feed"), data)
            .await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<TransactionFeed> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn list_transaction_feeds(&self) -> Result<Vec<TransactionFeed>, Error> {
        let response_byte = self
            .post(String::from("/create-transaction-feed"), String::new())
            .await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<Vec<TransactionFeed>> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn delete_transaction_feed(&self, alias: &str) -> Result<bool, Error> {
        let data = format!(r#"{{"alias":"{}"}}"#, alias);
        let response_byte = self
            .post(String::from("/delete-transaction-feed"), data)
            .await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<String> = serde_json::from_slice(response_byte.bytes())?;
        Ok(result.is_success())
    }

    pub async fn update_transaction_feed(&self, alias: &str, filter: &str) -> Result<bool, Error> {
        let data = format!(r#"{{"alias":"{}","filter":"{}"}}"#, alias, filter);
        let response_byte = self
            .post(String::from("/update-transaction-feed"), data)
            .await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<String> = serde_json::from_slice(response_byte.bytes())?;
        Ok(result.is_success())
    }

    pub async fn get_unconfirmed_transaction(&self, tx_id: &str) -> Result<BlockTx, Error> {
        let data = format!(r#"{{"tx_id":"{}"}}"#, tx_id);
        let response_byte = self
            .post(String::from("/get-unconfirmed-transaction"), data)
            .await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<BlockTx> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn list_unconfirmed_transaction(&self) -> Result<UnconfirmedTxsResp, Error> {
        let response_byte = self
            .post(
                String::from("/list-unconfirmed-transactions"),
                String::new(),
            )
            .await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<UnconfirmedTxsResp> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn decode_raw_transaction(&self, raw_transaction: &str) -> Result<RawTx, Error> {
        let data = format!(r#"{{"raw_transaction":"{}"}}"#, raw_transaction);
        let response_byte = self
            .post(String::from("/decode-raw-transaction"), data)
            .await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<RawTx> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn get_block_count(&self) -> Result<u64, Error> {
        let response_byte = self
            .post(String::from("/get-block-count"), String::new())
            .await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<HashMap<String, u64>> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            return match result.data.get("block_count") {
                Some(count) => Ok(*count),
                _ => Ok(0),
            };
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn get_block_hash(&self) -> Result<String, Error> {
        let response_byte = self
            .post(String::from("/get-block-hash"), String::new())
            .await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<HashMap<String, String>> =
            serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            return match result.data.get("block_hash") {
                Some(hash) => Ok(hash.to_string()),
                _ => Ok(String::new()),
            };
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn get_block(&self, height: u64, hash: &str) -> Result<GetBlockResp, Error> {
        let data = format!(r#"{{"block_height":{},"block_hash":"{}"}}"#, height, hash);
        let response_byte = self.post(String::from("/get-block"), data).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<GetBlockResp> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn get_block_header(
        &self,
        hash: &str,
        height: u64,
    ) -> Result<GetBlockHeaderResp, Error> {
        let data = format!(r#"{{"block_height":{},"block_hash":"{}"}}"#, height, hash);
        let response_byte = self.post(String::from("/get-block-header"), data).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<GetBlockHeaderResp> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn get_difficulty(
        &self,
        hash: &str,
        height: u64,
    ) -> Result<GetDifficultyResp, Error> {
        let data = format!(r#"{{"block_height":{},"block_hash":"{}"}}"#, height, hash);
        let response_byte = self.post(String::from("/get-difficulty"), data).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<GetDifficultyResp> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn get_hash_rate(&self, hash: &str, height: u64) -> Result<GetHashRateResp, Error> {
        let data = format!(r#"{{"block_height":{},"block_hash":"{}"}}"#, height, hash);
        let response_byte = self.post(String::from("/get-hash-rate"), data).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<GetHashRateResp> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn net_info(&self) -> Result<VersionInfo, Error> {
        let response_byte = self.post(String::from("/net-info"), String::new()).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<VersionInfo> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn is_mining(&self) -> Result<bool, Error> {
        let response_byte = self.post(String::from("/is-mining"), String::new()).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<HashMap<String, bool>> =
            serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            return match result.data.get("is_mining") {
                Some(mining) => Ok(*mining),
                _ => Ok(false),
            };
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn set_mining(&self, is_mining: bool) -> Result<bool, Error> {
        let data = format!(r#"{{"is_mining":{}}}"#, is_mining);
        let response_byte = self.post(String::from("/set-mining"), data).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<String> = serde_json::from_slice(response_byte.bytes())?;
        Ok(result.is_success())
    }

    pub async fn gas_rate(&self) -> Result<u64, Error> {
        let response_byte = self.post(String::from("/gas-rate"), String::new()).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<HashMap<String, u64>> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            return match result.data.get("gas_rate") {
                Some(rate) => Ok(*rate),
                _ => Ok(0),
            };
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn verify_message(
        &self,
        address: &str,
        derived_xpub: &str,
        message: &str,
        signature: &str,
    ) -> Result<bool, Error> {
        let data = format!(
            r#"{{"address":"{}","derived_xpub":"{}","message":"{}","signature":"{}"}}"#,
            address, derived_xpub, message, signature
        );
        let response_byte = self.post(String::from("/verify-message"), data).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<VerifyMsgResp> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data.result)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn compile(&self, param: &CompileReq) -> Result<ComileResp, Error> {
        let data = serde_json::to_string(param)?;
        let response_byte = self.post(String::from("/compile"), data).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<ComileResp> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn list_peers(&self) -> Result<Vec<PeerInfo>, Error> {
        let response_byte = self
            .post(String::from("/list-peers"), String::new())
            .await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<Vec<PeerInfo>> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn disconnect_peer(&self, peer_id: &str) -> Result<bool, Error> {
        let data = format!(r#"{{"peer_id":"{}"}}"#, peer_id);
        let response_byte = self.post(String::from("/list-peers"), data).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<String> = serde_json::from_slice(response_byte.bytes())?;
        Ok(result.is_success())
    }

    pub async fn connect_peer(&self, ip: &str, port: u16) -> Result<PeerInfo, Error> {
        let data = format!(r#"{{"ip":"{}","port":{}}}"#, ip, port);
        let response_byte = self.post(String::from("/connect-peer"), data).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<PeerInfo> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn get_work(&self) -> Result<GetWorkResp, Error> {
        let response_byte = self.post(String::from("/get-work"), String::new()).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<GetWorkResp> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn submit_work(&self, block_header: &str) -> Result<bool, Error> {
        let data = format!(r#"{{"block_header":"{}"}}"#, block_header);
        let response_byte = self.post(String::from("/submit-work"), data).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<bool> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn get_work_json(&self) -> Result<GetWorkJSONResp, Error> {
        let response_byte = self
            .post(String::from("/get-work-json"), String::new())
            .await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<GetWorkJSONResp> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }

    pub async fn submit_work_json(&self, param: &SubmitWorkJSONReq) -> Result<bool, Error> {
        let data = serde_json::to_string(param)?;
        let response_byte = self.post(String::from("/get-work-json"), data).await?;
        println!("response_byte {}", str::from_utf8(response_byte.bytes())?);
        let result: Response<bool> = serde_json::from_slice(response_byte.bytes())?;
        if result.is_success() {
            Ok(result.data)
        } else {
            Err(Error::from(BtmError::new(result)))
        }
    }
    async fn post(&self, path: String, data: String) -> Result<Bytes, Error> {
        let uri = Uri::builder()
            .scheme(self.scheme.as_str())
            .authority(self.host.as_str())
            .path_and_query(path.as_str())
            .build()?;
        let request = Request::builder()
            .method("POST")
            .uri(uri)
            .header("Countent-Type", "application/json")
            .body(Body::from(data))?;
        let response = self.client.request(request).await?;
        let response_byte = hyper::body::to_bytes(response.into_body()).await?;
        Ok(response_byte)
    }
}
