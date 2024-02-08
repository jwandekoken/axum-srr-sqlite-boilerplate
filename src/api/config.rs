use crate::infrastructure::modules::users::users_repository::UserRepository;

#[derive(Clone, Debug)]
pub struct AppState {
    pub user_repository: UserRepository,
}
