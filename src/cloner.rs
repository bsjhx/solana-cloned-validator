use solana_client::client_error;
use solana_client::rpc_client::RpcClient;
use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use base64::Engine;
use serde::Serialize;
use base64::prelude::BASE64_STANDARD;

pub fn get_account_json(address: &str) -> client_error::Result<AccountJson> {
    let result = get_account_data(address)?;

    Ok(AccountJson {
        pubkey: address.to_string(),
        account: AccountDetailsJson {
            data: vec![
                BASE64_STANDARD.encode(result.data),
                "base64".to_string(),
            ],
            executable: result.executable,
            lamports: result.lamports,
            owner: result.owner.to_string(),
            rent_epoch: result.rent_epoch,
        },
    })
}

fn get_account_data(address: &str) -> client_error::Result<Account> {
    let mainnet_rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    let address = Pubkey::from_str(&*address).unwrap();
    mainnet_rpc_client.get_account(&address)
}

#[derive(Serialize, Debug)]
pub struct AccountJson {
    pub pubkey: String,
    pub account: AccountDetailsJson,
}

#[derive(Serialize, Debug)]
pub struct AccountDetailsJson {
    pub data: Vec<String>,
    pub executable: bool,
    pub lamports: u64,
    pub owner: String,
    pub rent_epoch: u64,
}