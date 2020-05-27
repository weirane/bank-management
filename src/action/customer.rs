use crate::error::{Error, Result};
use sqlx::MySqlPool;

/// Updates the customer's `key` to `val`. Returns a BadRequest if the key is invalid.
#[rustfmt::skip]
pub async fn change(cus: &str, key: &str, val: &str, pool: &MySqlPool) -> Result<u64> {
    match key {
        "name" => {
            sqlx::query!("update customer set name = ? where customer_id = ?", val, cus)
                .execute(pool)
                .await
                .map_err(|e| e.into())
        }
        "tel" => {
            sqlx::query!("update customer set tel = ? where customer_id = ?", val, cus)
                .execute(pool)
                .await
                .map_err(|e| e.into())
        }
        "address" => {
            sqlx::query!("update customer set address = ? where customer_id = ?", val, cus)
                .execute(pool)
                .await
                .map_err(|e| e.into())
        }
        _ => Err(Error::BadRequest("invalid change field")),
    }
}
