use askama::Template;
use axum::response::IntoResponse;

use crate::domain::modules::users::user_model::User;

#[derive(Template)]
#[template(path = "users/list_users.html")]
struct ListUsersTemplate<'a> {
    title: &'a str,
    name: &'a str,
}

pub fn users(users: Vec<User>) -> impl IntoResponse {
    println!("Users: {:?}", users);

    ListUsersTemplate {
        title: "Users",
        name: "Test!",
    }; // instantiate your struct
}
