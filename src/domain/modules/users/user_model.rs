use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, FromRow, Debug, Deserialize, Serialize)]
pub struct User {
    id: Option<i16>,
    email: String,
    hashed_password: String,
    reset_password_selector: String,
    reset_password_sent_at: Option<String>,
    reset_password_validator_hash: Option<String>,
}
