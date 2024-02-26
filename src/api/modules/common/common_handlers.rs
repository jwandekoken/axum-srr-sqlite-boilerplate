use axum::{response::IntoResponse, Json};

use crate::ui::modules::common::views::home_view;

pub async fn home() -> impl IntoResponse {
    return home_view::execute();
}

pub async fn healthcheck() -> impl IntoResponse {
    const MESSAGE: &str = "Working fine, thanks!";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}
