use std::str::FromStr;

use solana_rpc::rpc::JsonRpcConfig;
use solana_sdk::pubkey::Pubkey;
use solana_test_validator::{AccountInfo, TestValidatorGenesis};

pub fn create_validator_genesis() -> TestValidatorGenesis {
    let mut jrc = JsonRpcConfig::default();
    jrc.enable_rpc_transaction_history = true;
    jrc.enable_extended_tx_metadata_storage = true;
    jrc.full_api = true;

    let mut test_validator = TestValidatorGenesis::default();
    test_validator.rpc_port(8899);
    test_validator.rpc_config(jrc);

    test_validator
}

pub fn add_accounts_to(test_validator: &mut TestValidatorGenesis) {
    let mut avec = Vec::<AccountInfo>::new();

    let usdc_pubkey = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
    avec.push(AccountInfo { address: Some(usdc_pubkey), filename: "usdc.json" });

    match test_validator.add_accounts_from_json_files(&avec){
        Ok(_) => println!("Account: {usdc_pubkey} added"),
        Err(error) => eprintln!("Couldn't add {usdc_pubkey} to test validator! Reason: {error}")
    }
}