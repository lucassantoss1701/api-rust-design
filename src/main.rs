use std::time::Duration;
use axum::{middleware, Router};
use axum::extract::Request;
use axum::http::{HeaderMap, StatusCode};
use axum::middleware::Next;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(header_handler))
        .route_layer(middleware::from_fn(auth))
        .fallback_service(ServeDir::new("web"))
        ;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    tokio::spawn(make_request());

    println!("Listening on 127.0.0.1:3001");
    axum::serve(listener, app).await.unwrap();
}

async fn header_handler(
    headers: HeaderMap
) -> Html<String>{
    if let Some(header) = headers.get("x-request-id"){
        Html(format!("x-request-id: {}", header.to_str().unwrap()))
    } else {
        Html("x-request-id not found".to_string())
    }
}

async fn make_request(){
    tokio::time::sleep(Duration::from_secs(1)).await;

    let response  = reqwest::Client::new()
        .get("http://localhost:3001/")
        .header("x-request-id",  "1234")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{}", response);


    let response  = reqwest::Client::new()
        .get("http://localhost:3001/")
        .header("x-request-id",  "bad")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{}", response);
}

async fn auth(
    headers: HeaderMap,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)>{
    if let Some(header) = headers.get("x-request-id"){
        if header.to_str().unwrap() == "1234"{
            return Ok(next.run(req).await)
        }
    }

    Err((StatusCode::UNAUTHORIZED, "invalid header".to_string()))
}


