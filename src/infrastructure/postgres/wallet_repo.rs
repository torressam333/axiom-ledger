use crate::domain::{Address, Balance, Wallet};
use crate::repository::WalletRepository;
use async_trait::async_trait;
use sqlx::PgPool;

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

    async fn save(&self, wallet: &Wallet) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO wallets (address, balance) VALUES ($1, $2) 
             ON CONFLICT (address) DO UPDATE SET balance = $2",
            wallet.address(),
            wallet.balance() as i64
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
