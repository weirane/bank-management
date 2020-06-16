use crate::error::Result;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Customer {
    pub id: String,
    pub name: String,
    pub tel: String,
    pub address: String,
}

#[derive(Debug, Deserialize)]
pub struct NewCustomer {
    pub id: String,
    pub name: String,
    pub tel: String,
    pub address: String,
    pub contacter_id: i32,
    pub relation: String,
}

impl NewCustomer {
    pub async fn add(&self, pool: &MySqlPool) -> Result<u64> {
        sqlx::query!(
            "insert into customer (customer_id, name, tel, address, contacter_id, relation)
             values (?, ?, ?, ?, ?, ?)",
            self.id,
            self.name,
            self.tel,
            self.address,
            self.contacter_id,
            self.relation
        )
        .execute(pool)
        .await
        .map_err(|e| e.into())
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum NewAccount {
    Save {
        id: String,
        bank: String,
        balance: BigDecimal,
        customers: Vec<String>,
        currency: String,
        interest_rate: BigDecimal,
    },
    Check {
        id: String,
        bank: String,
        balance: BigDecimal,
        customers: Vec<String>,
        credit: BigDecimal,
    },
}

impl NewAccount {
    pub async fn add(&self, pool: &MySqlPool) -> Result<()> {
        let (id, customers) = match self {
            NewAccount::Save {
                id,
                bank,
                balance,
                customers,
                currency,
                interest_rate,
            } => {
                sqlx::query!(
                    "call add_save_account(?, ?, ?, ?, ?)",
                    id,
                    bank,
                    balance,
                    currency,
                    interest_rate
                )
                .execute(pool)
                .await?;
                (id, customers)
            }
            NewAccount::Check {
                id,
                bank,
                balance,
                customers,
                credit,
            } => {
                sqlx::query!(
                    "call add_check_account(?, ?, ?, ?)",
                    id,
                    bank,
                    balance,
                    credit
                )
                .execute(pool)
                .await?;
                (id, customers)
            }
        };
        let fs = customers.iter().map(|c| {
            sqlx::query!(
                "insert into has_account (account_id, customer_id) values (?, ?)",
                id,
                c
            )
            .execute(pool)
        });
        futures::future::try_join_all(fs).await?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct NewLoan {
    // TODO
}

impl NewLoan {
    pub async fn add(&self, pool: &MySqlPool) -> Result<u64> {
        let _ = pool;
        todo!("NewLoan::add");
    }
}
