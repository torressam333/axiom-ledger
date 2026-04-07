use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};

#[async_trait]
pub trait TransactionProvider: Send + Sync {
    /// Starts new DB tx
    async fn begin_transaction(&self) -> Result<Transaction<'static, Postgres>, sqlx::Error>;
}

// PgPool must follow the rules
#[async_trait]
impl TransactionProvider for PgPool {
    async fn begin_transaction(&self) -> Result<Transaction<'static, Postgres>, sqlx::Error> {
        self.begin().await
    }
}
