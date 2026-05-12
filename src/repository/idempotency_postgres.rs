use crate::domain::LedgerError;
use crate::repository::IdempotencyRepository;
use crate::service::idempotent_service::TransferResponse;
use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::types::Json;
use uuid::Uuid;

pub struct PostgresIdempotencyRepository {
    pool: PgPool,
}

impl PostgresIdempotencyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IdempotencyRepository for PostgresIdempotencyRepository {
    async fn get_result(&self, uuid: Uuid) -> Result<Option<TransferResponse>, LedgerError> {
        // We use FOR UPDATE to lock the row.
        // If another request with the same UUID hits, it waits here.
        let row = sqlx::query!(
            r#"
        SELECT response_payload as "response_payload: Json<TransferResponse>"
        FROM processed_requests
        WHERE idempotency_key = $1
        FOR UPDATE
        "#,
            uuid
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| LedgerError::DatabaseError(e.to_string()))?;

        // 'r.response_payload' is an Option<Json<TransferResponse>>
        // We use .and_then(|json| Some(json.0)) to "flatten" it
        Ok(row.and_then(|r| r.response_payload).map(|json| json.0))
    }

    async fn save_result(
        &self,
        uuid: Uuid,
        response: &TransferResponse,
    ) -> Result<(), LedgerError> {
        sqlx::query!(
            r#"
            INSERT INTO processed_requests (idempotency_key, response_payload, status)
            VALUES ($1, $2, 'COMPLETED')
            ON CONFLICT (idempotency_key) 
            DO UPDATE SET 
                response_payload = EXCLUDED.response_payload,
                status = 'COMPLETED'
            "#,
            uuid,
            Json(response) as _
        )
        .execute(&self.pool)
        .await
        .map_err(|e| LedgerError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
