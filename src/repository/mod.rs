pub mod db_provider;
pub use db_provider::TransactionProvider;

use crate::domain::Address;
use crate::domain::Wallet;
use async_trait::async_trait;

#[async_trait]
pub trait WalletRepository: Send + Sync {
    type Tx;

    async fn find_by_address(
        &self,
        tx: &mut Self::Tx,
        address: &Address,
    ) -> Result<Option<Wallet>, sqlx::Error>;
    async fn save(&self, tx: &mut Self::Tx, wallet: &Wallet) -> Result<(), sqlx::Error>;

    async fn find_by_address_for_update(
        &self,
        tx: &mut Self::Tx,
        address: &Address,
    ) -> Result<Option<Wallet>, sqlx::Error>;
}
