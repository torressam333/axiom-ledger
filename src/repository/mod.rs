pub mod db_provider;
pub use db_provider::TransactionProvider;

use crate::domain::Address;
use crate::domain::Wallet;
use async_trait::async_trait;
use sqlx::{Postgres, Transaction};

#[async_trait]
pub trait WalletRepository: Send + Sync {
    async fn find_by_address(&self, address: &Address) -> Result<Option<Wallet>, sqlx::Error>;
    async fn save(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        wallet: &Wallet,
    ) -> Result<(), sqlx::Error>;

    async fn find_by_address_for_update(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        address: &Address,
    ) -> Result<Option<Wallet>, sqlx::Error>;
}
