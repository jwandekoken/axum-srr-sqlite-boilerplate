use axum::{routing::get, Router};

use crate::api::config::AppState;
use crate::api::modules::users::users_handlers;
use crate::infrastructure::modules::users::users_repository::UserRepository;

pub fn create_user_routes(user_repository: UserRepository) -> Router {
    let app_state = AppState { user_repository };

    Router::new()
        .route("/", get(users_handlers::list_users))
        .with_state(app_state)
}
