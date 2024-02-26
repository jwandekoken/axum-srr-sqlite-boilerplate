use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeViewTemplate<'a> {
    name: &'a str,
}

pub fn execute() -> impl IntoResponse {
    let template = HomeViewTemplate { name: "John Doe!!" };
    template
}
