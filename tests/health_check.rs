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

#[test]
fn withdrawing_from_balance_with_insufficient_funds_errors() {
    let intial_balance = Balance::new(100);
    let withdraw_amount = Balance::new(1000);
    let address = "rPT1Sjq2YGrvB3yS2ne8heJWTVyK3u6mcw";
    let mut wallet = Wallet::new(address, intial_balance, Currency::XRP).unwrap();

    let result = wallet.withdraw(withdraw_amount);

    // Shouold get error
    assert!(result.is_err());

    // Balance shouldn't have changed
    assert_eq!(wallet.balance(), intial_balance.value())
}

#[test]
fn test_balance_from_xrp_conversion() {
    let test_cases = vec![
        ("1", 1_000_000, "Whole numbers"),
        ("1.5", 1_500_000, "Decimals"),
        ("0.000001", 1, "Minimum drop"),
        ("100.25", 100_250_000, "Large amounts"),
    ];

    for (input, expected_drops, description) in test_cases {
        let balance = Balance::from_xrp(input).expect(description);

        assert_eq!(
            balance.value(),
            expected_drops,
            "Failed on {}.",
            description
        );
    }
}

#[test]
fn test_balance_from_xrp_invalid_formats() {
    let test_cases = vec!["1.2.3", "abc", "1.0000005", " 100abcw.    "];

    for input in test_cases {
        assert!(Balance::from_xrp(input).is_err());
    }
}
