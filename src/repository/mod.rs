use crate::domain::Address;
use crate::domain::Wallet;
use async_trait::async_trait;

// So rust does support AFIT or async fn's in traits since 1.75.
// I'm on 1.91 but I think we still need the crate for Dyn traits
#[async_trait]
pub trait WalletRepository: Send + Sync {
    // Returns Option b/c a wallet might not exist
    async fn find_by_address(&self, address: &Address) -> Result<Option<Wallet>, sqlx::Error>;

    // Standardized signature for saving a wallet
    async fn save(&self, wallet: &Wallet) -> Result<(), sqlx::Error>;
}
