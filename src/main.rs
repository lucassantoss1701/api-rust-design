use std::collections::HashMap;
use std::fmt::format;
use axum::{response::Html, routing::get, Json, Router};
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

struct MyState(i32);
fn service_one() -> Router{
    let state= Arc::new(MyState(5));
    Router::new().route("/", get(sv1_handler))
        .with_state(state)
}

async fn sv1_handler(
    Extension(counter): Extension<Arc<MyCounter>>,
    State(state): State<Arc<MyState>>
) -> Html<String>{
    counter.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Html(format!("Service {}-{}",
                 counter.counter.load(std::sync::atomic::Ordering::Relaxed),
                 state.0
    ))
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

        .route("/", get(handler))
        .route("/book/:id", get(path_extract))
        .route("/book", get(query_extract))
        .route("/header", get(header_extract))
        .route("/inc", get(increment))
        .with_state(shared_counter)
        .nest("/1", service_one())
        .nest("/2", service_two())
        // .layer(Extension(shared_counter))
        .layer(Extension(shared_text));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("Listening on 127.0.0.1:3001");
    axum::serve(listener, app).await.unwrap();
}

async fn increment(State(counter): State<Arc<MyCounter>>) -> Json<usize>{
    println!("/inc called");
   let current_value =  counter.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Json(current_value)
}

// async fn handler(
//     Extension(counter): Extension<Arc<MyCounter>>,
//     Extension(config): Extension<Arc<MyConfig>>
// ) -> Html<String>{
//     counter.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
//     Html(format!("<h1> {} You are visitior number {} </h1>",
//                  config.text,
//                  counter.counter.load(std::sync::atomic::Ordering::Relaxed)))
// }

async fn handler() -> Html<String>{
    println!("Sending GET request");
    let current_count = reqwest::get("http://localhost:3001/inc")
        .await
        .unwrap()
        .json::<i32>()
        .await
        .unwrap();

    Html(format!("<h1>> Remote counter: {current_count} </h1>"))
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