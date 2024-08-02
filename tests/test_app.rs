use std::str::FromStr;

use solana_cloned_validator::runner::*;
use solana_sdk::pubkey::Pubkey;

#[test]
fn test_if_solana_node_is_started_and_account_is_cloned() {
    let mut test_validator = create_validator_genesis(None);
    add_accounts_to(&mut test_validator);
    let (test_validator, _k) = test_validator.start();

    let rpc_client = test_validator.get_rpc_client();

    let usdc_pubkey = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
    let usdc_account = rpc_client.get_account(&usdc_pubkey);

    assert!(usdc_account.is_ok());
}
