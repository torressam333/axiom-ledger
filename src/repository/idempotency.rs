use crate::domain::LedgerError;
use crate::service::idempotent_service::TransferResponse;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait IdempotencyRepository: Send + Sync {
    // Add an executor parameter to allow passing a Transaction for TX managemet
    async fn get_result(&self, uuid: Uuid) -> Result<Option<TransferResponse>, LedgerError>;

    // Take a mutable reference to the connection/transaction
    async fn save_result(
        &self,
        executor: &mut sqlx::PgConnection,
        uuid: Uuid,
        response: &TransferResponse,
    ) -> Result<(), LedgerError>;
}
