use axum::{
    routing::{get, post},
    Router,
};
use rust_llm_api::routes::chat::chat_handler;
use rust_llm_api::state::AppState;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let state = AppState::new();

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/chat", post(chat_handler))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
