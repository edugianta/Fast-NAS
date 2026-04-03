use reqwest;

const APP_URL:&str = "http:/127.0.0.1:3000/health_check";

#[tokio::test]
async fn test_health_check() {
    let response = reqwest::get(APP_URL).await.unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::OK);
}
