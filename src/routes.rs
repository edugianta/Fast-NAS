use axum::http;
use axum::response::Html;

pub async fn root() -> Html<&'static str> {
    Html("<h1>Hi</h1>")
}


pub async fn health_check() -> http::StatusCode {
    http::StatusCode::OK
}