use crate::error::Result;
use sqlx::MySqlPool;

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct Customer {
    pub id: String,
    pub name: String,
    pub tel: String,
    pub address: String,
}

#[derive(Debug, serde::Deserialize)]
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
