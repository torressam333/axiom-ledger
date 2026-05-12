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
    pool: sqlx::PgPool,
}

#[async_trait]
impl<T: TransferService> TransferService for IdempotentTransferService<T> {
    async fn execute(&self, request: TransferRequest) -> Result<TransferResponse, LedgerError> {
        let key = request.idempotency_key;

        // 1. Check for existing result (outside the main transfer tx to keep it fast)
        if let Some(existing) = self.repo.get_result(key).await? {
            return Ok(existing);
        }

        // 2. Start a Transaction for the "Real Work"
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| LedgerError::DatabaseError(e.to_string()))?;

        // 3. Execute the actual transfer
        // Note: You'll eventually want to update your inner service
        // to also accept &mut tx for true atomicity!
        let response = self.inner.execute(request).await?;

        // 4. Save the idempotency record within the SAME transaction
        self.repo.save_result(&mut *tx, key, &response).await?;

        // 5. Commit! If we fail here, the transfer and the key save both vanish.
        tx.commit()
            .await
            .map_err(|e| LedgerError::DatabaseError(e.to_string()))?;

        Ok(response)
    }
}
