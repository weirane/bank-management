use crate::error::{Error, Result};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use std::convert::TryInto;

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

// Sadly we can only have String's in nested enums for serde_urlencoded.
//   https://github.com/nox/serde_urlencoded/issues/26
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum NewAccount {
    Save {
        id: String,
        bank: String,
        balance: String,
        currency: String,
        interest_rate: String,
    },
    Check {
        id: String,
        bank: String,
        balance: String,
        credit: String,
    },
}

impl TryInto<TypedNewAccount> for NewAccount {
    type Error = crate::error::Error;
    fn try_into(self) -> Result<TypedNewAccount> {
        let r = match self {
            NewAccount::Save {
                id,
                bank,
                balance,
                currency,
                interest_rate,
            } => {
                if currency.len() != 3 {
                    return Err(Error::BadRequest("currency format"));
                }
                TypedNewAccount::Save {
                    id,
                    bank,
                    currency,
                    balance: balance.parse().map_err(|_| Error::BadRequest("balance"))?,
                    interest_rate: interest_rate
                        .parse()
                        .map_err(|_| Error::BadRequest("interest_rate"))?,
                }
            }
            NewAccount::Check {
                id,
                bank,
                balance,
                credit,
            } => TypedNewAccount::Check {
                id,
                bank,
                balance: balance.parse().map_err(|_| Error::BadRequest("balance"))?,
                credit: credit.parse().map_err(|_| Error::BadRequest("credit"))?,
            },
        };
        Ok(r)
    }
}

#[derive(Debug)]
pub enum TypedNewAccount {
    Save {
        id: String,
        bank: String,
        balance: BigDecimal,
        currency: String,
        interest_rate: BigDecimal,
    },
    Check {
        id: String,
        bank: String,
        balance: BigDecimal,
        credit: BigDecimal,
    },
}

impl TypedNewAccount {
    pub async fn add(&self, pool: &MySqlPool) -> Result<()> {
        match self {
            TypedNewAccount::Save {
                id,
                bank,
                balance,
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
            }
            TypedNewAccount::Check {
                id,
                bank,
                balance,
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
            }
        }
        // TODO: update has_account for customers
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
