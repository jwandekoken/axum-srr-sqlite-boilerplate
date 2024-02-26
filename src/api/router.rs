use axum::Router;
use sqlx::{Pool, Sqlite};

use crate::api::modules::common::common_routes::create_common_routes;
use crate::api::modules::users::users_routes::create_user_routes;
use crate::infrastructure::modules::users::users_repository::SqlxUserRepository;

pub fn create_router(pool: Pool<Sqlite>) -> Router {
    let common_routes = create_common_routes();

    let user_repository = SqlxUserRepository::new(pool.clone());
    let user_routes = create_user_routes(user_repository);

    let routes = Router::new()
        .nest("/", user_routes)
        .nest("/", common_routes);

    Router::new().nest("/", routes)
}
