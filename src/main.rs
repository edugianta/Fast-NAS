mod routes;
mod state;
mod file;
mod errors;

use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {

    let operating_system = std::env::consts::OS;

    let app = Router::new()
        .route("/", get(routes::root))
        .route("/health_check", get(routes::health_check));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    println!("Current OS: {}", std::env::consts::OS);

    axum::serve(listener, app).await;
}

