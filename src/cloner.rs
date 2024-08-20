use std::fs::File;
use std::path::Path;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use serde::{Deserialize, Serialize};
use solana_client::client_error;
use solana_client::rpc_client::RpcClient;
use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub fn clone_accounts_to_files(destination: &str, addresses: Vec<String>) {
    let path = Path::new(destination);

    if !path.is_dir() {
        panic!("Given path is not a directory")
    }

    if !path.exists() {
        std::fs::create_dir(path).expect("Couldn't create folder");
    }

    for address in addresses.iter() {
        let account = get_account_json(address).unwrap();
        let json_path = format!("{}.json", address);
        let json_file = File::create(path.join(&json_path)).expect(&format!("Couldn't create file {}", &json_path));
        serde_json::to_writer(json_file, &account).unwrap();
    }
}

fn get_account_json(address: &str) -> client_error::Result<AccountJson> {
    let result = get_account_data(address)?;

    Ok(AccountJson {
        pubkey: address.to_string(),
        account: AccountDetailsJson {
            data: vec![BASE64_STANDARD.encode(result.data), "base64".to_string()],
            executable: result.executable,
            lamports: result.lamports,
            owner: result.owner.to_string(),
            rent_epoch: result.rent_epoch,
        },
    })
}

fn get_account_data(address: &str) -> client_error::Result<Account> {
    let mainnet_rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    let address = Pubkey::from_str(address).unwrap();
    mainnet_rpc_client.get_account(&address)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountJson {
    pub pubkey: String,
    pub account: AccountDetailsJson,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountDetailsJson {
    pub data: Vec<String>,
    pub executable: bool,
    pub lamports: u64,
    pub owner: String,
    pub rent_epoch: u64,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use tempfile::TempDir;
    use super::*;

    #[test]
    fn test_foo() {
        // given
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let temp_dir_path = temp_dir.path().to_str().unwrap();

        // when
        clone_accounts_to_files(temp_dir_path, vec!["EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string()]);

        // then
        let file_path = Path::new(temp_dir_path).join("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v.json");
        let contents = fs::read_to_string(&file_path).expect("Failed to read file");
        let actual: AccountJson = serde_json::from_str(&contents).unwrap();

        assert_eq!(
            actual.pubkey,
            "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
        );
        assert_eq!(actual.account.data.len(), 2);
        assert_eq!(actual.account.data[1], "base64");
        assert_eq!(
            actual.account.owner,
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        );
        assert_eq!(actual.account.executable, false);
    }
}