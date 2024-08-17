use solana_cloned_validator::cloner::get_account_json;

#[test]
fn test_reading_account_data_from_mainnet() {
    let actual = get_account_json("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
    assert!(actual.is_ok());

    let actual = actual.unwrap();

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
