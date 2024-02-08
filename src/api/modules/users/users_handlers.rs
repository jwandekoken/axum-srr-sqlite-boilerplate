use axum::{extract::State, response::IntoResponse, Json};

use crate::api::config::AppState;
use crate::application::modules::users::list_users_usecase;

pub async fn list_users(State(state): State<AppState>) -> impl IntoResponse {
    let users = list_users_usecase::list_users(&state.user_repository).await;
    let json_response = serde_json::json!(users);
    Json(json_response)
}
