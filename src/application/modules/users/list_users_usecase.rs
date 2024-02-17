use crate::domain::modules::users::user_model::User;
use crate::infrastructure::modules::users::users_repository::UserRepository;
use futures::executor::block_on;

pub fn list_users(user_repository: &dyn UserRepository) -> Vec<User> {
    let user_list_future = user_repository.get_all_users();

    let user_list = match block_on(user_list_future) {
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
