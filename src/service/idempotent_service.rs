use crate::domain::{Address, Balance, Currency, LedgerError};
use crate::repository::IdempotencyRepository;
use async_trait::async_trait;

pub struct TransferRequest {
    pub idempotency_key: uuid::Uuid,
    pub from: Address,
    pub to: Address,
    pub amount: Balance,
    pub currency: Currency,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TransferResponse {
    pub transaction_id: String,
    pub status: String,
}

#[async_trait]
pub trait TransferService: Send + Sync {
    async fn execute(&self, request: TransferRequest) -> Result<TransferResponse, LedgerError>;
}

pub struct IdempotentTransferService<T: TransferService> {
    inner: T,
    repo: Box<dyn IdempotencyRepository>,
}

#[async_trait]
impl<T: TransferService> TransferService for IdempotentTransferService<T> {
    async fn execute(&self, request: TransferRequest) -> Result<TransferResponse, LedgerError> {
        let key = request.idempotency_key;

        // 1. Check if key exists
        if let Some(existing) = self.repo.get_result(request.idempotency_key).await? {
            return Ok(existing);
        }

        // 2. If valid, call inner service
        let response = self.inner.execute(request).await?;

        // 3. Save result to DB
        self.repo.save_result(key, &response).await?;

        // 4. Return result
        Ok(response)
    }
}
