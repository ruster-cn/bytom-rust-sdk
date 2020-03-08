use bytom_rust_sdk::client::BtmClient;
use bytom_rust_sdk::account::request::CreateAccountRequestParam;
use bytom_rust_sdk::transaction::request::{CreateAssetRequestParam, ListUnspentOutputsRequestParam};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    create_key().await;
}

#[allow(dead_code)]
async fn create_key() {
    let client = BtmClient::new("47.103.115.37:9888".to_string(), "http".to_string());
    client
        .create_key("delete-key", "delete-key", "en", "")
        .await
        .map_err(|err| {
            println!("{}", err.to_string());
        })
        .map(|response| println!("{:?}", response))
        .ok();
}

#[allow(dead_code)]
async fn list_keys() {
    let client = BtmClient::new("47.103.115.37:9888".to_string(), "http".to_string());
    client
        .list_keys()
        .await
        .map(|response| println!("{:?}", response))
        .map_err(|err| println!("{:?}", err.to_string()) )
        .ok();
}

#[allow(dead_code)]
async fn update_key_alias() {
    let client = BtmClient::new("47.103.115.37:9888".to_string(), "http".to_string());
    client
        .update_key_alias("940f37b6f8b26d4bbd59e6c3907b0e2f55a574db07bc210210a3b6df5dce54935b72a2dd899e0aab6da61ac953a24ad74e812f32816305339bf2ed86ccb9207f","uyi")
        .await.map(|response| println!("{:?}", response))
        .map_err(|err| println!("{:?}", err.to_string()) )
        .ok();

}

#[allow(dead_code)]
async fn delete_key(){
    let client = BtmClient::new("47.103.115.37:9888".to_string(), "http".to_string());
    client
        .delete_key("940f37b6f8b26d4bbd59e6c3907b0e2f55a574db07bc210210a3b6df5dce54935b72a2dd899e0aab6da61ac953a24ad74e812f32816305339bf2ed86ccb9207f","delete-key")
        .await
        .map(|response| println!("{:?}",response))
        .map_err(|err|println!("{:?}",err.to_string()))
        .ok();
}

#[allow(dead_code)]
async fn check_key_password(){
    let client = BtmClient::new("47.103.115.37:9888".to_string(), "http".to_string());
    client
        .check_key_password("9d25b9c4e767d1fe1c78dc306339b4f2133681d5e1fe6f6555e3398837eb86dbf4b632c711c9d322ada44de4619518fe5c1a62f641fa2f695204f7e558c7d531","delete-key")
        .await
        .map(|response| println!("{:?}",response))
        .map_err(|err|println!("{:?}",err.to_string()))
        .ok();
}


#[allow(dead_code)]
async fn reset_key_password(){
    let client = BtmClient::new("47.103.115.37:9888".to_string(), "http".to_string());
    client
        .reset_key_password("9d25b9c4e767d1fe1c78dc306339b4f2133681d5e1fe6f6555e3398837eb86dbf4b632c711c9d322ada44de4619518fe5c1a62f641fa2f695204f7e558c7d531","delete-key","reset-key")
        .await
        .map(|response| println!("{:?}",response))
        .map_err(|err|println!("{:?}",err.to_string()))
        .ok();
}


#[allow(dead_code)]
async fn create_account() {
    let client = BtmClient::new("47.103.115.37:9888".to_string(), "http".to_string());
    let request_param = CreateAccountRequestParam{
        root_xpubs:vec!["2d6c07cb1ff7800b0793e300cd62b6ec5c0943d308799427615be451ef09c0304bee5dd492c6b13aaa854d303dc4f1dcb229f9578786e19c52d860803efa3b9a".to_string()],
        alias: "bob".to_string(),
        quorum: 1
    };
    client
        .create_account(&request_param)
        .await
        .map(|response| println!("{:?}",response))
        .map_err(|err|println!("{:?}",err.to_string()))
        .ok();
}

#[allow(dead_code)]
async fn list_account(){
    let client = BtmClient::new("47.103.115.37:9888".to_string(), "http".to_string());
    client
        .list_account("","test")
        .await
        .map(|response| println!("{:?}",response))
        .map_err(|err|println!("{:?}",err.to_string()))
        .ok();
}

#[allow(dead_code)]
async fn update_account_alias(){
    let client = BtmClient::new("47.103.115.37:9888".to_string(), "http".to_string());
    client
        .update_account_alias("dad","bob","18CTVQUV00A04")
        .await
        .map(|response| println!("{:?}",response))
        .map_err(|err|println!("{:?}",err.to_string()))
        .ok();
}


#[allow(dead_code)]
async fn delete_account() {
    let client = BtmClient::new("47.103.115.37:9888".to_string(), "http".to_string());
    client
        .delete_account("dad", "")
        .await
        .map(|response| println!("{:?}",response))
        .map_err(|err|println!("{:?}",err.to_string()))
        .ok();
}

#[allow(dead_code)]
async fn create_account_receiver() {
    let client = BtmClient::new("47.103.115.37:9888".to_string(), "http".to_string());
    client
        .create_account_receiver("test", "")
        .await
        .map(|response| println!("{:?}",response))
        .map_err(|err|println!("{:?}",err.to_string()))
        .ok();
}

#[allow(dead_code)]
async fn list_address() {
    let client = BtmClient::new("47.103.115.37:9888".to_string(), "http".to_string());
    client
        .list_address("test", "18CTR8QRG0A02", 0, 3)
        .await
        .map(|response| println!("{:?}",response))
        .map_err(|err|println!("{:?}",err.to_string()))
        .ok();
}

#[allow(dead_code)]
async fn validate_address() {
    let client = BtmClient::new("47.103.115.37:9888".to_string(), "http".to_string());
    client
        .validate_address("sm1q8vp4nrzp3e6pryu6kyn34gu3y3qxgpuyaqqnlq")
        .await
        .map(|response| println!("{:?}",response))
        .map_err(|err|println!("{:?}",err.to_string()))
        .ok();
}

#[allow(dead_code)]
async fn get_mining_address(){
    let client = BtmClient::new("47.103.115.37:9888".to_string(), "http".to_string());
    client
        .get_mining_address()
        .await
        .map(|response| println!("{:?}",response))
        .map_err(|err|println!("{:?}",err.to_string()))
        .ok();
}

#[allow(dead_code)]
async fn set_mining_address(){
    let client = BtmClient::new("47.103.115.37:9888".to_string(), "http".to_string());
    client
        .set_mining_address("sm1q5aqlsgu542rkqefxumhejlwl7awvgtxrk259gd")
        .await
        .map(|response| println!("{:?}",response))
        .map_err(|err|println!("{:?}",err.to_string()))
        .ok();
}

#[allow(dead_code)]
async fn get_coinbase_arbitrary(){
    let client = BtmClient::new("47.103.115.37:9888".to_string(), "http".to_string());
    client
        .get_coinbase_arbitrary()
        .await
        .map(|response| println!("{:?}",response))
        .map_err(|err|println!("{:?}",err.to_string()))
        .ok();
}

#[allow(dead_code)]
async fn set_coinbase_arbitrary(){
    let client = BtmClient::new("47.103.115.37:9888".to_string(), "http".to_string());
    client
        .set_coinbase_arbitrary("ff")
        .await
        .map(|response| println!("{:?}",response))
        .map_err(|err|println!("{:?}",err.to_string()))
        .ok();
}

#[allow(dead_code)]
async fn list_pubkeys(){
    let client = BtmClient::new("47.103.115.37:9888".to_string(), "http".to_string());
    client
        .list_pubkeys("test","","")
        .await
        .map(|response| println!("{:?}",response))
        .map_err(|err|println!("{:?}",err.to_string()))
        .ok();
}

#[allow(dead_code)]
async fn create_asset(){
    let client = BtmClient::new("47.103.115.37:9888".to_string(), "http".to_string());
    let param = CreateAssetRequestParam{
        alias: "yuo".to_string(),
        root_xpubs: vec!["f8a60c06c35fad4ac994aaa545e91f5f6d64a72539982d332976df46d16b4f78d72b8564b7b5d0f95220293ddf0a20c29f310f8c209b52af6fa4c2726416b194".to_string()],
        quorum: 1,
        definition: Default::default(),
        limit_height: 0,
        issuance_program: "".to_string()
    };
    client
        .create_asset(&param)
        .await
        .map(|response| println!("{:?}",response))
        .map_err(|err|println!("{:?}",err.to_string()))
        .ok();

    let mut map = HashMap::new();
    map.insert("ee",1);
}

#[allow(dead_code)]
async fn get_asset(){
    let client = BtmClient::new("47.103.115.37:9888".to_string(), "http".to_string());

    client
        .get_asset("2ade6c7370b3e2e2d9b7be4f5806a0f3d12a79bf8b0d6a171c91026a8fa3a8c2")
        .await
        .map(|response| println!("{:?}",response))
        .map_err(|err|println!("{:?}",err.to_string()))
        .ok();

    let mut map = HashMap::new();
    map.insert("ee",1);
}

#[allow(dead_code)]
async fn list_asset(){
    let client = BtmClient::new("47.103.115.37:9888".to_string(), "http".to_string());

    client
        .list_assets("")
        .await
        .map(|response| println!("{:?}",response))
        .map_err(|err|println!("{:?}",err.to_string()))
        .ok();

    let mut map = HashMap::new();
    map.insert("ee",1);
}
#[allow(dead_code)]
async fn update_asset_alas(){
    let client = BtmClient::new("47.103.115.37:9888".to_string(), "http".to_string());

    client
        .update_asset_alias("2ade6c7370b3e2e2d9b7be4f5806a0f3d12a79bf8b0d6a171c91026a8fa3a8c2","dongxu")
        .await
        .map(|response| println!("{:?}",response))
        .map_err(|err|println!("{:?}",err.to_string()))
        .ok();

    let mut map = HashMap::new();
    map.insert("ee",1);
}

#[allow(dead_code)]
async fn list_balance(){
    let client = BtmClient::new("47.103.115.37:9888".to_string(), "http".to_string());

    client
        .list_balances("17QPQEHJ00A04","alice")
        .await
        .map(|response| println!("{:?}",response))
        .map_err(|err|println!("{:?}",err.to_string()))
        .ok();

    let mut map = HashMap::new();
    map.insert("ee",1);
}

#[allow(dead_code)]
async fn list_unspent_outputs(){
    let client = BtmClient::new("47.103.115.37:9888".to_string(), "http".to_string());
    let param = ListUnspentOutputsRequestParam{
        account_id: "17QPQEHJ00A04".to_string(),
        account_alias: "alice".to_string(),
        id: "".to_string(),
        unconfirmed: false,
        smart_contract: false,
        from: 0,
        count: 0
    };
    client
        .list_unspent_outputs(&param)
        .await
        .map(|response| println!("{:?}",response))
        .map_err(|err|println!("{:?}",err.to_string()))
        .ok();

    let mut map = HashMap::new();
    map.insert("ee",1);
}


#[allow(dead_code)]
async fn backup_wallet(){
    let client = BtmClient::new("47.103.115.37:9888".to_string(), "http".to_string());

    client
        .backup_wallet()
        .await
        .map(|response| println!("{:?}",response))
        .map_err(|err|println!("{:?}",err.to_string()))
        .ok();

    let mut map = HashMap::new();
    map.insert("ee",1);
}