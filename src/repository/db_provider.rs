use async_trait::async_trait;
use sqlx::PgPool;

// db_provider.rs
pub struct PostgresTx(pub(crate) sqlx::Transaction<'static, sqlx::Postgres>);

#[async_trait]
pub trait TransactionProvider: Send + Sync {
    type Tx;

    async fn begin_transaction(&self) -> Result<Self::Tx, anyhow::Error>;
}

#[async_trait]
pub trait TransactionHandler: Send {
    async fn commit(self) -> Result<(), anyhow::Error>;
}

// PgPool must follow the rules
#[async_trait]
impl TransactionProvider for PgPool {
    type Tx = PostgresTx;

    async fn begin_transaction(&self) -> Result<Self::Tx, anyhow::Error> {
        let tx = self.begin().await?;

        Ok(PostgresTx(tx))
    }
}

#[async_trait]
impl TransactionHandler for PostgresTx {
    async fn commit(self) -> Result<(), anyhow::Error> {
        self.0.commit().await.map_err(|e| anyhow::anyhow!(e))
    }
}
