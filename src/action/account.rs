use crate::error::{Error, Result};
use crate::types::{CheckAccount, SaveAccount};

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

pub async fn query_save(form: &crate::route::SMap, pool: &MySqlPool) -> Result<Vec<SaveAccount>> {
    let empty = String::new();
    let customer = form.get("customer").unwrap_or(&empty);
    if !customer.is_empty() {
        sqlx::query_as!(
            SaveAccount,
            "select distinct account_id as id, bank, balance,
                substr('储蓄账户支票账户', type*4+1, 4) as type_,
                open_date, interest_rate, currency
            from (
                select account_id, customer_id
                from has_account left join customer
                using(customer_real_id)
            ) x right join saveaccounts using(account_id) where
            customer_id like concat('%', ?, '%')
            and account_id like concat('%', ?, '%')
            and bank like concat('%', ?, '%')
            and balance like concat('%', ?, '%')
            and open_date like concat('%', ?, '%')
            and interest_rate like concat('%', ?, '%')
            and currency like concat('%', ?, '%')
            ",
            customer,
            form.get("id").unwrap_or(&empty),
            form.get("bank").unwrap_or(&empty),
            form.get("balance").unwrap_or(&empty),
            form.get("open_date").unwrap_or(&empty),
            form.get("interest_rate").unwrap_or(&empty),
            form.get("currency").unwrap_or(&empty),
        )
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    } else {
        sqlx::query_as!(
            SaveAccount,
            "select account_id as id, bank, balance,
                substr('储蓄账户支票账户', type*4+1, 4) as type_,
                open_date, interest_rate, currency from saveaccounts where
            account_id like concat('%', ?, '%')
            and bank like concat('%', ?, '%')
            and balance like concat('%', ?, '%')
            and open_date like concat('%', ?, '%')
            and interest_rate like concat('%', ?, '%')
            and currency like concat('%', ?, '%')
            ",
            form.get("id").unwrap_or(&empty),
            form.get("bank").unwrap_or(&empty),
            form.get("balance").unwrap_or(&empty),
            form.get("open_date").unwrap_or(&empty),
            form.get("interest_rate").unwrap_or(&empty),
            form.get("currency").unwrap_or(&empty),
        )
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    }
}

pub async fn query_check(form: &crate::route::SMap, pool: &MySqlPool) -> Result<Vec<CheckAccount>> {
    let empty = String::new();
    let customer = form.get("customer").unwrap_or(&empty);
    if !customer.is_empty() {
        sqlx::query_as!(
            CheckAccount,
            "select account_id as id, bank, balance,
                substr('储蓄账户支票账户', type*4+1, 4) as type_,
                open_date, credit
            from (
                select account_id, customer_id
                from has_account left join customer
                using(customer_real_id)
            ) x right join checkaccounts using(account_id) where
            customer_id like concat('%', ?, '%')
            and account_id like concat('%', ?, '%')
            and bank like concat('%', ?, '%')
            and balance like concat('%', ?, '%')
            and open_date like concat('%', ?, '%')
            and credit like concat('%', ?, '%')
            ",
            customer,
            form.get("id").unwrap_or(&empty),
            form.get("bank").unwrap_or(&empty),
            form.get("balance").unwrap_or(&empty),
            form.get("open_date").unwrap_or(&empty),
            form.get("credit").unwrap_or(&empty),
        )
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    } else {
        sqlx::query_as!(
            CheckAccount,
            "select account_id as id, bank, balance,
                substr('储蓄账户支票账户', type*4+1, 4) as type_,
                open_date, credit from checkaccounts where
            account_id like concat('%', ?, '%')
            and bank like concat('%', ?, '%')
            and balance like concat('%', ?, '%')
            and open_date like concat('%', ?, '%')
            and credit like concat('%', ?, '%')
            ",
            form.get("id").unwrap_or(&empty),
            form.get("bank").unwrap_or(&empty),
            form.get("balance").unwrap_or(&empty),
            form.get("open_date").unwrap_or(&empty),
            form.get("credit").unwrap_or(&empty),
        )
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    }
}
