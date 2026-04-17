use crate::domain::{Address, Balance, Currency, Wallet};
use crate::repository::WalletRepository;
use crate::repository::db_provider::PostgresTx;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

#[derive(Clone)]
pub struct PostgresWalletRepository {
    pool: PgPool,
}

impl PostgresWalletRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl WalletRepository for PostgresWalletRepository {
    type Tx = PostgresTx; // Tell rust we're using Postgres TX

    async fn find_by_address(
        &self,
        tx: &mut Self::Tx,
        address: &Address,
    ) -> Result<Option<Wallet>, sqlx::Error> {
        let addr_str = address.as_str();

        let row = sqlx::query!(
            "SELECT address, balance, currency FROM wallets WHERE address = $1",
            addr_str
        )
        .fetch_optional(&mut *tx.0) // <--- "Peel the onion" once to get the inner TX
        .await?;

        if let Some(r) = row {
            // PARSE, DON'T VALIDATE
            let currency = match r.currency.as_str() {
                "XRP" => Currency::XRP,
                _ => return Err(sqlx::Error::Decode("Unknown currency in DB".into())),
            };

            let wallet = Wallet::new(&r.address, Balance::new(r.balance as u128), currency)
                .map_err(|e| sqlx::Error::Decode(e.into()))?;

            Ok(Some(wallet))
        } else {
            Ok(None)
        }
    }

    async fn save(&self, tx: &mut Self::Tx, wallet: &Wallet) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO wallets (address, balance, currency) VALUES ($1, $2, $3) 
             ON CONFLICT (address) DO UPDATE SET balance = $2",
            wallet.address(),
            wallet.balance() as i64,
            wallet.currency().as_str()
        )
        .execute(&mut *tx.0) // Gets us access to raw DB conn. 1. -> (Transaction(DB Conn)) -> 2. Transaction(DB Conn) -> 3. DB Conn
        .await?;

        Ok(())
    }

    async fn find_by_address_for_update(
        &self,
        tx: &mut Self::Tx,
        address: &Address,
    ) -> Result<Option<Wallet>, sqlx::Error> {
        let row = sqlx::query!(
            r#"
        SELECT address, balance, currency
        FROM wallets
        WHERE address = $1
        FOR UPDATE
        "#,
            address.as_str()
        )
        .fetch_optional(&mut *tx.0)
        .await?;

        // If we find a row turn into a wallet. Otherwise, do nothing.
        Ok(row.map(|r| {
            Wallet::new(&r.address, Balance::new(r.balance as u128), Currency::XRP).unwrap()
        }))
    }
}
