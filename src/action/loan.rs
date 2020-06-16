use crate::error::Result;
use sqlx::MySqlPool;

pub async fn del(loan: &str, pool: &MySqlPool) -> Result<u64> {
    sqlx::query!("delete from loan where loan_id = ?", loan)
        .execute(pool)
        .await
        .map_err(Into::into)
}

