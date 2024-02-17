use futures::Future;
use sqlx::{Pool, Sqlite};
use std::pin::Pin;

use crate::domain::modules::users::user_model::User;

pub trait UserRepository {
    fn get_all_users(&self) -> Pin<Box<dyn Future<Output = Result<Vec<User>, sqlx::Error>> + '_>>;
}

#[derive(Clone, Debug)]
pub struct SqlxUserRepository {
    pool: Pool<Sqlite>,
}

impl SqlxUserRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    async fn get_all_users_impl(&self) -> Result<Vec<User>, sqlx::Error> {
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

impl UserRepository for SqlxUserRepository {
    fn get_all_users(&self) -> Pin<Box<dyn Future<Output = Result<Vec<User>, sqlx::Error>> + '_>> {
        Box::pin(self.get_all_users_impl())
    }
}
