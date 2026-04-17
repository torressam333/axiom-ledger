use axiom_ledger::domain::Address;
use axiom_ledger::infrastructure::postgres::PostgresWalletRepository;
use axiom_ledger::repository::TransactionProvider;
use axiom_ledger::repository::WalletRepository;
use axiom_ledger::repository::db_provider::TransactionHandler;
use axiom_ledger::service::TransferService;
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;

#[tokio::test]
async fn test_execute_transfer_success_integration() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await.unwrap();

    let alice_addr = Address::new("rAlicePT1Sjq2YGrBMTU2C27uPHqc9S7fP".to_string()).unwrap();
    let bob_addr = Address::new("rBobPT1Sjq2YGrBMTU2C27uPHqc9S7fPfM".to_string()).unwrap();

    let repo = PostgresWalletRepository::new(pool.clone());
    let service = TransferService::new(repo.clone(), pool.clone());

    // 1. Setup - Using &pool here is fine because this is raw SQL setup, not the Repo
    sqlx::query!(
        "INSERT INTO wallets (address, balance, currency) VALUES ($1, $2, $3)
         ON CONFLICT (address) DO UPDATE SET balance = $2, currency = $3",
        alice_addr.as_str(),
        100i64,
        "XRP"
    )
    .execute(&pool)
    .await
    .unwrap();

    // 2. Execute
    let result = service.execute_transfer(&alice_addr, &bob_addr, 30).await;
    assert!(result.is_ok());

    // 3. Verify - WE NEED A TX TO USE THE REPO
    // We call the provider (pool) to get a fresh verification transaction
    let mut verify_tx = pool.begin_transaction().await.unwrap();

    let alice_after = repo
        .find_by_address(&mut verify_tx, &alice_addr)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(alice_after.balance(), 70);

    // We don't strictly need to commit verify_tx because we didn't change data,
    // but it's good practice to close the "baton" properly.
    verify_tx.commit().await.unwrap();
}

// NEED to test the failure case...prove that the atomoicity works
#[tokio::test]
async fn test_execute_transfer_fails_insufficient_funds_rolls_back() {
    dotenv().ok();
    let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    let repo = PostgresWalletRepository::new(pool.clone());
    let service = TransferService::new(repo.clone(), pool.clone());

    let alice_addr = Address::new("rAlicePT1Sjq2YGrBMTU2C27uPHqc9S7fP".to_string()).unwrap();
    let bob_addr = Address::new("rBobPT1Sjq2YGrBMTU2C27uPHqc9S7fPfM".to_string()).unwrap();

    // 1. Reset Alice to 100 XRP
    sqlx::query!(
        "INSERT INTO wallets (address, balance, currency) VALUES ($1, $2, $3)
         ON CONFLICT (address) DO UPDATE SET balance = $2",
        alice_addr.as_str(),
        100i64,
        "XRP"
    )
    .execute(&pool)
    .await
    .unwrap();

    // 2. Attempt to transfer 1000 XRP (Alice only has 100)
    let result = service.execute_transfer(&alice_addr, &bob_addr, 1000).await;

    // 3. The service should return an error
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Insufficient funds")
    );

    // 4. ATOMIC PROOF: Alice's balance must STILL be 100
    let mut verify_tx = pool.begin_transaction().await.unwrap();

    let alice_after = repo
        .find_by_address(&mut verify_tx, &alice_addr)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(alice_after.balance(), 100);

    verify_tx.commit().await.unwrap();

    assert_eq!(alice_after.balance(), 100);
}
