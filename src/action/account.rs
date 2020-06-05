use crate::error::Result;

use sqlx::MySqlPool;

pub async fn del(id: &str, pool: &MySqlPool) -> Result<u64> {
    sqlx::query!("delete from account where account_id = ?", id)
        .execute(pool)
        .await
        .map_err(Into::into)
}
