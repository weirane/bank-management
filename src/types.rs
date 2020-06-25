use crate::error::Result;
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Customer {
    pub id: String,
    pub name: String,
    pub tel: String,
    pub address: String,
    pub contacter_id: String,
    pub relation: String,
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

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SaveAccount {
    pub id: String,
    pub bank: String,
    pub open_date: NaiveDate,
    pub balance: BigDecimal,
    #[serde(rename = "type")]
    pub type_: String,
    pub currency: String,
    pub interest_rate: BigDecimal,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CheckAccount {
    pub id: String,
    pub bank: String,
    pub open_date: NaiveDate,
    pub balance: BigDecimal,
    #[serde(rename = "type")]
    pub type_: String,
    pub credit: BigDecimal,
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

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Loan {
    pub id: String,
    pub bank: String,
    pub amount: BigDecimal,
    pub state: String,
}

#[derive(Debug, Deserialize)]
pub struct NewLoan {
    id: String,
    amount: BigDecimal,
    bank: String,
    customers: Vec<String>,
}

impl NewLoan {
    pub async fn add(&self, pool: &MySqlPool) -> Result<()> {
        sqlx::query!(
            "insert into loan (loan_id, amount, bank) values (?, ?, ?)",
            self.id,
            self.amount,
            self.bank
        )
        .execute(pool)
        .await?;
        let fs = self.customers.iter().map(|c| {
            sqlx::query!(
                "insert into make_loan (loan_id, customer_id) values (?, ?)",
                self.id,
                c
            )
            .execute(pool)
        });
        futures::future::try_join_all(fs).await?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct NewLoanPay {
    id: String,
    amount: BigDecimal,
}

impl NewLoanPay {
    pub async fn add(&self, pool: &MySqlPool) -> Result<u64> {
        sqlx::query!(
            "insert into loan_pay (loan_id, amount) values (?, ?)",
            self.id,
            self.amount,
        )
        .execute(pool)
        .await
        .map_err(Into::into)
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SaveStat {
    pub bank: String,
    pub total_balance: BigDecimal,
    pub total_customer: i64,
}

impl SaveStat {
    pub fn no_business(&self) -> bool {
        self.total_balance == 0f64.into() && self.total_customer == 0
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct LoanStat {
    pub bank: String,
    pub total_loanpay: BigDecimal,
    pub total_customer: i64,
}

impl LoanStat {
    pub fn no_business(&self) -> bool {
        self.total_loanpay == 0f64.into() && self.total_customer == 0
    }
}
