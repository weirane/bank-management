use crate::error::{Error, Result};
use sqlx::MySqlPool;

/// Deletes the customer with the ID of `cus`.
pub async fn del(cus: &str, pool: &MySqlPool) -> Result<u64> {
    sqlx::query!("delete from customer where customer_id = ?", cus)
        .execute(pool)
        .await
        .map_err(Into::into)
}

/// Updates the customer's `key` to `val`. Returns a BadRequest if the key is invalid.
#[rustfmt::skip]
pub async fn change(cus: i32, key: &str, val: &str, pool: &MySqlPool) -> Result<u64> {
    match key {
        "customer_id" => {
            sqlx::query!("update customer set customer_id = ? where customer_real_id = ?", val, cus)
                .execute(pool)
                .await
                .map_err(Into::into)
        }
        "name" => {
            sqlx::query!("update customer set name = ? where customer_real_id = ?", val, cus)
                .execute(pool)
                .await
                .map_err(Into::into)
        }
        "tel" => {
            sqlx::query!("update customer set tel = ? where customer_real_id = ?", val, cus)
                .execute(pool)
                .await
                .map_err(Into::into)
        }
        "address" => {
            sqlx::query!("update customer set address = ? where customer_real_id = ?", val, cus)
                .execute(pool)
                .await
                .map_err(Into::into)
        }
        "contacter_id" => {
            sqlx::query!("update customer set contacter_id = ? where customer_real_id = ?", val, cus)
                .execute(pool)
                .await
                .map_err(Into::into)
        }
        "relation" => {
            sqlx::query!("update customer set relation = ? where customer_real_id = ?", val, cus)
                .execute(pool)
                .await
                .map_err(Into::into)
        }
        _ => Err(Error::BadRequest("invalid change field")),
    }
}
