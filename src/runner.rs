use std::{
    fs::{self, FileTimes},
    path::Path,
    str::FromStr,
};

use solana_rpc::rpc::JsonRpcConfig;
use solana_sdk::pubkey::Pubkey;
use solana_test_validator::{AccountInfo, TestValidatorGenesis};

pub fn create_validator_genesis(port: Option<u16>) -> TestValidatorGenesis {
    let jrc = create_rpc_config();

    let mut test_validator = TestValidatorGenesis::default();
    test_validator.rpc_config(jrc);

    if let Some(port) = port {
        test_validator.rpc_port(port);
    }

    test_validator
}

pub fn add_accounts_to(test_validator: &mut TestValidatorGenesis, path: &str) {
    let path = Path::new(path);

    if !path.exists() || !path.is_dir() {
        panic!("Given path does not exists or is not a directory")
    }

    let files = get_files_path_from(path);
    let files = match files {
        Ok(files) => files,
        Err(err) => panic!(
            "Couldn't add accounts to validator, problem with: {:?}",
            err
        ),
    };
    let avec: Vec<AccountInfo> = files
        .into_iter()
        .map(|file_path| AccountInfo {
            address: None,
            filename: Box::leak(file_path.into_boxed_str()),
        })
        .collect();

    let a = test_validator.add_accounts_from_json_files(&avec);
    println!("ddd");
}

fn get_files_path_from(path: &Path) -> std::io::Result<Vec<String>> {
    let mut files_path = vec![];

    for entry in fs::read_dir(path)? {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                if let Some(path_str) = path.to_str() {
                    files_path.push(path_str.to_string());
                }
            }
        }
    }

    Ok(files_path)
}

fn create_rpc_config() -> JsonRpcConfig {
    JsonRpcConfig {
        enable_rpc_transaction_history: true,
        enable_extended_tx_metadata_storage: true,
        full_api: true,
        ..Default::default()
    }
}
