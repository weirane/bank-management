use crate::error::{Error, Result};

use sqlx::MySqlPool;

pub async fn del(id: &str, pool: &MySqlPool) -> Result<u64> {
    sqlx::query!("delete from account where account_id = ?", id)
        .execute(pool)
        .await
        .map_err(Into::into)
}

#[rustfmt::skip]
pub async fn change_save(id: &str, key: &str, val: &str, pool: &MySqlPool) -> Result<u64> {
    match key {
        "interest_rate" => {
            sqlx::query!("update saveacc set interest_rate = ? where account_id = ?", val, id)
                .execute(pool)
                .await
                .map_err(Into::into)
        }
        "currency" => {
            sqlx::query!("update saveacc set currency = ? where account_id = ?", val, id)
                .execute(pool)
                .await
                .map_err(Into::into)
        }
        _ => Err(Error::BadRequest("invalid change field")),
    }
}

#[rustfmt::skip]
pub async fn change_check(id: &str, key: &str, val: &str, pool: &MySqlPool) -> Result<u64> {
    match key {
        "credit" => {
            sqlx::query!("update checkacc set credit = ? where account_id = ?", val, id)
                .execute(pool)
                .await
                .map_err(Into::into)
        }
        _ => Err(Error::BadRequest("invalid change field")),
    }
}
