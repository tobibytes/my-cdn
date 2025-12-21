use axum::http::{
    HeaderValue, Method,
    header::{ACCEPT, ACCESS_CONTROL_ALLOW_CREDENTIALS, AUTHORIZATION, CONTENT_TYPE, COOKIE},
};
use axum::{
    Router,
    response::{IntoResponse, Json, Response},
    routing::{get, post},
};
mod models;
use tower_http::cors::CorsLayer;
mod controllers;
mod services;
use controllers::{media::{upload_complete_handler, upload_init_handler}, media_controller_init};
use tracing::info;
use tracing_subscriber::EnvFilter;
#[tokio::main]
async fn main() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).with_target(false).init();

    let port = std::env::var("PORT").expect("PORT not set");
    let frontend_url = std::env::var("FRONTEND_URL").expect("FRONTEND_URL not set");
    info!(%port, %frontend_url, "starting service");
    let allowed_origin: HeaderValue = frontend_url.parse().expect("Invalid FRONTEND_URL for CORS");
    let cors = CorsLayer::new()
        .allow_origin(allowed_origin)
        .allow_credentials(true)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            AUTHORIZATION,
            CONTENT_TYPE,
            ACCEPT,
            COOKIE,
            ACCESS_CONTROL_ALLOW_CREDENTIALS,
        ]);
        // dependencies
        let media_controller = media_controller_init();
    let app = Router::new()
        .route("/", get(root))
        .route("/upload/init", post(upload_init_handler))
        .route("/upload/complete", post(upload_complete_handler))
        .layer(cors)
        .with_state(media_controller);
    info!(%port, "running");
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn root() -> Response {
    (
        Json(String::from("Welcome to url shortener")),
    )
        .into_response()
}
