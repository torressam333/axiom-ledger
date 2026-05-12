use crate::domain::LedgerError;
use crate::service::idempotent_service::TransferResponse;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait IdempotencyRepository: Send + Sync {
    async fn get_result(&self, uuid: Uuid) -> Result<Option<TransferResponse>, LedgerError>;
    async fn save_result(&self, uuid: Uuid, response: &TransferResponse)
    -> Result<(), LedgerError>;
}
