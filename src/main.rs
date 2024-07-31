use std::{str::FromStr, time::Duration};
use solana_cloned_validator::runner::{add_accounts_to, create_validator_genesis};
use solana_sdk::pubkey::Pubkey;

fn main() {
    let mut test_validator = create_validator_genesis();
    add_accounts_to(&mut test_validator);

    let (test_validator, _k) = test_validator.start();

    let rpc_url = test_validator.rpc_url();
    println!("Test validator RPC URL: {}", rpc_url);

    let rpc_client = test_validator.get_rpc_client();

    loop {
        let slot = rpc_client.get_slot().unwrap();
        println!("Current slot: {slot}");
        std::thread::sleep(Duration::from_secs(1));
        let usdc_pubkey = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
        let a = rpc_client.get_token_supply(&usdc_pubkey).unwrap();
        println!("hh {:?}", a);
    }
}
