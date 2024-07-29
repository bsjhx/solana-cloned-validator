use std::{str::FromStr, time::Duration};

use solana_rpc::rpc::JsonRpcConfig;
use solana_sdk::pubkey::Pubkey;
use solana_test_validator::{AccountInfo, TestValidatorGenesis};

fn main() {
    let mut jrc = JsonRpcConfig::default();
    jrc.enable_rpc_transaction_history = true;
    jrc.enable_extended_tx_metadata_storage = true;
    jrc.full_api = true;

    let mut test_validator = TestValidatorGenesis::default();
    test_validator.rpc_port(8899);
    test_validator.rpc_config(jrc);

    let mut avec = Vec::<AccountInfo>::new();

    let usdc_pubkey = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
    avec.push(AccountInfo { address: Some(usdc_pubkey), filename: "usdc.json" });

    test_validator.add_accounts_from_json_files(&avec);

    let (test_validator, _k) = test_validator.start();

    let rpc_url = test_validator.rpc_url();
    println!("Test validator RPC URL: {}", rpc_url);

    let rpc_client = test_validator.get_rpc_client();

    loop {
        println!("Sleep");
        std::thread::sleep(Duration::from_secs(1));
    }
}

