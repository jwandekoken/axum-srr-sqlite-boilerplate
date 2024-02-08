use std::net::SocketAddr;

mod api;
mod application;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let pool = infrastructure::db::connect()
        .await
        .expect("Failed to initialize database pool");

    let app = api::router::create_router(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
