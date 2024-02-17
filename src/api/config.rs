use crate::infrastructure::modules::users::users_repository::SqlxUserRepository;

#[derive(Clone, Debug)]
pub struct AppState {
    pub user_repository: SqlxUserRepository,
}
