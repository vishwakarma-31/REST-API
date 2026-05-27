use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDate;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Employee {
    pub id: i32,
    pub name: String,
    pub department: String,
    pub role: String,
    pub salary: Decimal,
    pub location: String,
    pub joined_at: NaiveDate,
}
