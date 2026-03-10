use axiom_ledger::domain::{Balance, Currency, Wallet};

#[test]
fn wallet_creation_works_for_xrpl() {
    // Setup part. Using 128 for mathematical precision. No room for error in fintech.
    let initial_balance = Balance::new(100_000_000);
    let address = "rPT1Sjq2YGrvB3yS2ne8heJWTVyK3u6mcw";

    // Execution part
    let wallet = Wallet::new(address, initial_balance, Currency::XRP).unwrap();

    // Verification
    assert_eq!(wallet.balance(), 100_000_000);
    assert_eq!(wallet.address(), address);
}

#[test]
fn wallet_rejects_invalid_xrpl_address() {
    let initial_balance = Balance::new(1_000_000);

    // https://xrpl.org/docs/concepts/accounts/addresses -> Doesn't begin with "r" for example
    let invalid_address = "not-an-xrpl-address";

    let result = Wallet::new(invalid_address, initial_balance, Currency::XRP);

    // We expect this to be an Error now, not a valid Wallet
    assert!(result.is_err());
}

#[test]
fn wallet_deposit_updates_balance() {
    let initial_balance = Balance::new(100); // XRP drops and using type driven design for safety
    let address = "rPT1Sjq2YGrvB3yS2ne8heJWTVyK3u6mcw";
    let mut wallet = Wallet::new(address, initial_balance, Currency::XRP).unwrap();

    let additional_balance = Balance::new(50);

    wallet.deposit(additional_balance);

    // Wallet shouold have additional funds after depositing
    assert_eq!(wallet.balance(), 150);
}
