use std::{
    sync::{atomic::AtomicU64, Arc},
    time::{Duration, Instant},
};

use axum::{extract::State, response::IntoResponse, routing::get, Router};

#[derive(Debug, Clone)]
struct CounterState {
    start: Arc<Instant>,
    counter: Arc<AtomicU64>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .with_state(CounterState {
            start: Arc::new(Instant::now()),
            counter: Default::default(),
        });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root(State(state): State<CounterState>) -> impl IntoResponse {
    tokio::time::sleep(Duration::from_secs(1)).await;

    let count = state
        .counter
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    if count > 1010 {
        println!(
            " - request: {: >4}, elapsed: {:.4}",
            count,
            state.start.elapsed().as_secs_f32()
        );
    }

    "done"
}