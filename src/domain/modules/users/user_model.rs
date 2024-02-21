use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, FromRow, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Option<i16>,
    pub email: String,
    pub hashed_password: String,
    pub reset_password_selector: String,
    pub reset_password_sent_at: Option<String>,
    pub reset_password_validator_hash: Option<String>,
}
