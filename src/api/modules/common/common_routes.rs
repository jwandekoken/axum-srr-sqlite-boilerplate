use axum::{routing::get, Router};
use tower_http::services::ServeDir;

use crate::api::modules::common::common_handlers;

pub fn create_common_routes() -> Router {
    let assets_path = std::env::current_dir().unwrap();

    Router::new()
        .route("/healthcheck", get(common_handlers::healthcheck))
        .route("/", get(common_handlers::home))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        )
}
