use askama::Template;
use axum::response::IntoResponse;

use crate::domain::modules::users::user_model::User;

#[derive(Template)]
#[template(path = "users/list_users.html")]
struct ListUsersTemplate<'a> {
    name: &'a str,
}

pub fn users(users: Vec<User>) -> impl IntoResponse {
    println!("Users: {:?}", users);

    let template = ListUsersTemplate { name: "John Doe!" };

    template
}
