use axiom_ledger::domain::{Currency, Wallet};

#[test]
fn wallet_creation_works_for_xrpl() {
    // Setup part. Using 128 for mathematical precision. No room for error in fintech.
    let initial_balance: u128 = 100_000_000;
    let address = "rPT1Sjq2YGrvB3yS2ne8heJWTVyK3u6mcw";

    // Execution part
    let wallet = Wallet::new(address, initial_balance, Currency::XRP);

    // Verification
    assert_eq!(wallet.balance(), 100_000_000);
    assert_eq!(wallet.address(), address);
}
