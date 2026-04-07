use crate::domain::{Address, Balance, Currency, Wallet};
use crate::repository::WalletRepository;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};

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
    async fn find_by_address(&self, address: &Address) -> Result<Option<Wallet>, sqlx::Error> {
        let addr_str = address.as_str();

        let row = sqlx::query!(
            "SELECT address, balance FROM wallets WHERE address = $1",
            addr_str
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| {
            let balance_val = r.balance as u128;
            // Note: Mapping DB i64 back to our Domain u128
            Wallet::new(
                &r.address,
                Balance::new(balance_val),
                crate::domain::wallet::Currency::XRP,
            )
            .unwrap()
        }))
    }

    async fn save(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        wallet: &Wallet,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO wallets (address, balance) VALUES ($1, $2) 
             ON CONFLICT (address) DO UPDATE SET balance = $2",
            wallet.address(),
            wallet.balance() as i64
        )
        .execute(&mut **tx) // Gets us access to raw DB conn. 1. -> (Transaction(DB Conn)) -> 2. Transaction(DB Conn) -> 3. DB Conn
        .await?;

        Ok(())
    }

    async fn find_by_address_for_update(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        address: &Address,
    ) -> Result<Option<Wallet>, sqlx::Error> {
        let row = sqlx::query!(
            r#"
        SELECT address, balance
        FROM wallets
        WHERE address = $1
        FOR UPDATE
        "#,
            address.as_str()
        )
        .fetch_optional(&mut **tx)
        .await?;

        // If we find a row turn into a wallet. Otherwise, do nothing.
        Ok(row.map(|r| {
            Wallet::new(&r.address, Balance::new(r.balance as u128), Currency::XRP).unwrap()
        }))
    }
}
