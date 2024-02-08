use sqlx::{Pool, Sqlite};

use crate::domain::modules::users::user_model::User;

#[derive(Clone, Debug)]
pub struct UserRepository {
    pool: Pool<Sqlite>,
}

impl UserRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let user_results = match sqlx::query_as::<_, User>(
            r#"
        SELECT *
        FROM users
        "#,
        )
        .fetch_all(&self.pool)
        .await
        {
            Ok(results) => Ok(results),
            Err(e) => {
                // Specific error handling based on 'e'
                eprintln!("Database error: {}", e);
                // Convert to your function's error type
                Err(e.into())
            }
        };

        user_results
    }
}
