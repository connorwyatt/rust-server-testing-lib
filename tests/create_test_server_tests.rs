use std::str::FromStr;

use axum::{routing::get, Router};
use hyper::{Client, Uri};
use server_testing::create_test_server;

#[tokio::test]
async fn create_test_server_should_respond_to_requests() {
    let socket_addr = create_test_server(app());

    let client = Client::new();

    let uri = Uri::from_str(format!("http://localhost:{}/test", socket_addr.port()).as_str())
        .expect("uri should be parseable");

    let result = client.get(uri).await;

    assert!(result.is_ok());

    assert_eq!(result.unwrap().status(), hyper::http::StatusCode::OK);
}

fn app() -> Router {
    Router::new().route("/test", get(handle))
}

async fn handle() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}
