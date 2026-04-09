use axiom_ledger::domain::Address;
use axiom_ledger::infrastructure::postgres::PostgresWalletRepository;
use axiom_ledger::repository::WalletRepository;
use axiom_ledger::service::TransferService;
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;

#[tokio::test]
async fn test_execute_transfer_success_integration() {
    dotenv().ok(); // This looks for a .env file and loads it into the process

    //Get conn string from .env then connect to local db (Im using postgres)
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env or environment");
    let pool = PgPool::connect(&db_url).await.unwrap();

    // Create user addresses FIRST
    let alice_addr_str = "rAlicePT1Sjq2YGrBMTU2C27uPHqc9S7fP";
    let bob_addr_str = "rBobPT1Sjq2YGrBMTU2C27uPHqc9S7fPfM";

    // Init the repo and the service
    let repo = PostgresWalletRepository::new(pool.clone());
    let service = TransferService::new(repo.clone(), pool.clone());

    // Prep the data for this tx
    let alice_addr = Address::new(alice_addr_str.to_string()).unwrap();
    let bob_addr = Address::new(bob_addr_str.to_string()).unwrap();

    // (Insert Alice with 100 and Bob with 50 into DB
    // Save their wallets and an XRP balance into the db
    sqlx::query!(
        "INSERT INTO wallets (address, balance, currency) VALUES ($1, $2, $3)
         ON CONFLICT (address) DO UPDATE SET balance = $2, currency = $3",
        alice_addr.as_str(), // $1
        100i64,              // $2 (Postgres usually wants i64 for BIGINT)
        "XRP"                // $3
    )
    .execute(&pool)
    .await
    .expect("Failed to insert Alice's wallet");

    sqlx::query!(
        "INSERT INTO wallets (address, balance, currency) VALUES ($1, $2, $3)
         ON CONFLICT (address) DO UPDATE SET balance = $2, currency = $3",
        bob_addr.as_str(),
        50i64,
        "XRP"
    )
    .execute(&pool)
    .await
    .expect("Failed to insert Bob's wallet");

    // Execute the transfer
    let result = service.execute_transfer(&alice_addr, &bob_addr, 30).await;

    assert!(result.is_ok());

    // Verify Alice has 70 and Bob has 80 in the DB now
    let alice_after = repo.find_by_address(&alice_addr).await.unwrap().unwrap();
    assert_eq!(alice_after.balance(), 70);
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
    let alice_after = repo.find_by_address(&alice_addr).await.unwrap().unwrap();
    assert_eq!(alice_after.balance(), 100);
}
