use crate::error::Result;
use crate::types::Loan;
use sqlx::MySqlPool;

pub async fn del(loan: &str, pool: &MySqlPool) -> Result<u64> {
    sqlx::query!("delete from loan where loan_id = ?", loan)
        .execute(pool)
        .await
        .map_err(Into::into)
}

pub async fn query(form: &crate::route::SMap, pool: &MySqlPool) -> Result<Vec<Loan>> {
    let empty = String::new();
    let customer = form.get("customer").unwrap_or(&empty);
    if !customer.is_empty() {
        sqlx::query_as!(
            Loan,
            "select loan_id as id, bank, amount, paid,
                substr('未开始发放正在发放 已全部发放', state*5+1, 5) as state
            from (
                select loan_id, customer_id
                from make_loan left join customer
                using(customer_real_id)
            ) x right join loan_with_paid using(loan_id) where
            customer_id like concat('%', ?, '%')
            and loan_id like concat('%', ?, '%')
            and bank like concat('%', ?, '%')
            and amount like concat('%', ?, '%')
            and state like concat('%', ?, '%')
            ",
            customer,
            form.get("id").unwrap_or(&empty),
            form.get("bank").unwrap_or(&empty),
            form.get("amount").unwrap_or(&empty),
            form.get("state").unwrap_or(&empty),
        )
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    } else {
        sqlx::query_as!(
            Loan,
            "select loan_id as id, bank, amount, paid,
                substr('未开始发放正在发放 已全部发放', state*5+1, 5) as state
            from loan_with_paid where
            loan_id like concat('%', ?, '%')
            and bank like concat('%', ?, '%')
            and amount like concat('%', ?, '%')
            and state like concat('%', ?, '%')
            ",
            form.get("id").unwrap_or(&empty),
            form.get("bank").unwrap_or(&empty),
            form.get("amount").unwrap_or(&empty),
            form.get("state").unwrap_or(&empty),
        )
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    }
}
