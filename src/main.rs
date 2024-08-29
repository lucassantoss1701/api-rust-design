use axum::{Json, Router};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("Listening on 127.0.0.1:3001");
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Result<impl IntoResponse, StatusCode>{
    let start = std::time::SystemTime::now();

    let seconds_wrapped = start
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.as_secs() % 3;

    let divided = 100u64.checked_div(seconds_wrapped)
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(divided))

}

