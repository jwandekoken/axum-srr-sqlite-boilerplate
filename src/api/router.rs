use axum::{response::IntoResponse, routing::get, Json, Router};
use sqlx::{Pool, Sqlite};

use crate::api::modules::users::users_routes::create_user_routes;
use crate::infrastructure::modules::users::users_repository::SqlxUserRepository;

async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Working fine, thanks!";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

pub fn create_router(pool: Pool<Sqlite>) -> Router {
    let user_repository = SqlxUserRepository::new(pool.clone());

    let user_routes = create_user_routes(user_repository);

    let common_routes = Router::new().route("/healthcheck", get(health_checker_handler));

    let api_routes = Router::new()
        .nest("/users", user_routes)
        .nest("/", common_routes);

    Router::new().nest("/api", api_routes)
}
