use std::collections::HashMap;
use axum::{response::Html, routing::get, Router};
use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use axum::Extension;

struct MyCounter{
    counter: AtomicUsize
}

struct MyConfig{
    text: String
}

fn service_one() -> Router{
    Router::new().route("/", get(|| async {Html("Service one".to_string())}))
}

fn service_two() -> Router{
    Router::new().route("/", get(|| async {Html("Service two".to_string())}))
}


#[tokio::main]
async fn main() {
    let shared_counter = Arc::new(MyCounter {
        counter: AtomicUsize::new(0)
    });

    let shared_text = Arc::new(MyConfig{
        text: "This is my configuration".to_string()
    });

    let app = Router::new()
        .nest("/1", service_one())
        .nest("/2", service_two())
        .route("/", get(handler))
        .route("/book/:id", get(path_extract))
        .route("/book", get(query_extract))
        .route("/header", get(header_extract))
        .layer(Extension(shared_counter))
        .layer(Extension(shared_text));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("Listening on 127.0.0.1:3001");
    axum::serve(listener, app).await.unwrap();
}

async fn handler(
    Extension(counter): Extension<Arc<MyCounter>>,
    Extension(config): Extension<Arc<MyConfig>>
) -> Html<String>{
    counter.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Html(format!("<h1> {} You are visitior number {} </h1>",
                 config.text,
                 counter.counter.load(std::sync::atomic::Ordering::Relaxed)))
}

async fn path_extract(
    Path(id): Path<u32>
) -> Html<String>{
    Html(format!("Hello, {id}!"))
}

async fn query_extract(
   Query(params): Query<HashMap<String, String>>
) -> Html<String>{
    Html(format!("Hello, {params:#?}!"))
}

async fn header_extract(
    headers: HeaderMap
) -> Html<String>{
    Html(format!("{headers:#?}"))
}