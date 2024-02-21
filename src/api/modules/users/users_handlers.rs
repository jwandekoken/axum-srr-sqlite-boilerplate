use axum::{extract::State, response::Html};

use crate::api::config::AppState;
use crate::application::modules::users::list_users_usecase;

use crate::api::modules::users::views::list_users_view;

// pub async fn list_users(State(state): State<AppState>) -> impl IntoResponse {
//     let users = list_users_usecase::list_users(&state.user_repository);
//     let json_response = serde_json::json!(users);
//     Json(json_response)
// }

pub async fn list_users(State(state): State<AppState>) -> Html<String> {
    let users = list_users_usecase::list_users(&state.user_repository);

    println!("Users: {:?}", users);

    Html(list_users_view::users(users))
}
