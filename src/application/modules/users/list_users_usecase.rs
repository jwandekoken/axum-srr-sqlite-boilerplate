use crate::domain::modules::users::user_model::User;
use crate::infrastructure::modules::users::users_repository::UserRepository;

pub async fn list_users(user_repository: &UserRepository) -> Vec<User> {
    let user_list = match user_repository.get_all_users().await {
        Ok(results) => results,
        Err(e) => {
            // Specific error handling based on 'e'
            eprintln!("Database error: {}", e);
            // @TODO: handle this error?
            Vec::new()
        }
    };

    user_list
}
