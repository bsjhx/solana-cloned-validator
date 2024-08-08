use std::str::FromStr;

use solana_cloned_validator::cloner::{get_account_json};
use solana_cloned_validator::runner::*;
use solana_sdk::pubkey::Pubkey;

#[test]
fn test_if_solana_node_is_started_and_account_is_cloned() {
    let mut test_validator = create_validator_genesis(None);
    let result = add_accounts_to(&mut test_validator, "tests/resources");

    assert!(result.is_ok());

    let (test_validator, _k) = test_validator.start();

    let rpc_client = test_validator.get_rpc_client();

    let usdc_pubkey = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
    let usdc_account = rpc_client.get_account(&usdc_pubkey);

    assert!(usdc_account.is_ok());

    let metaplex_pubkey = Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").unwrap();
    let metaplex_account = rpc_client.get_account(&metaplex_pubkey);

    assert!(metaplex_account.is_ok());
}

#[test]
#[should_panic(expected = "Given path does not exists or is not a directory")]
fn test_if_path_does_not_exists_should_panic() {
    let mut test_validator = create_validator_genesis(None);
    let _ = add_accounts_to(&mut test_validator, "./not_existing_path");
}

#[test]
#[should_panic(expected = "Given path does not exists or is not a directory")]
fn test_if_path_is_not_a_file_should_panic() {
    let mut test_validator = create_validator_genesis(None);
    let _ = add_accounts_to(&mut test_validator, "./resources/usdc.json");
}

#[test]
fn test_reading_account_data_from_mainnet() {
    let actual = get_account_json("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
    assert!(actual.is_ok());

    let actual = actual.unwrap();

    assert_eq!(actual.pubkey, "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
    assert_eq!(actual.account.data.len(), 2);
    assert_eq!(actual.account.data[1], "base64");
    assert_eq!(actual.account.owner, "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
    assert_eq!(actual.account.executable, false);
}
