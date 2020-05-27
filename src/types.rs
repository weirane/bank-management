#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct Customer {
    pub id: String,
    pub name: String,
    pub tel: String,
    pub address: String,
}
