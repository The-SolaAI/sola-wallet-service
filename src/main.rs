mod routes;
use axum::{
    http::Method, Router
};

use tower_http::{
    cors::CorsLayer,
    cors::Any,
    trace::TraceLayer
};


#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .nest("/api", routes::router::swap_router())
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}