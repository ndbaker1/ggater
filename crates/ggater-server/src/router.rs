use std::sync::{Arc, Mutex};

use axum::{routing::get, Extension, Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::{services::ServeDir, trace::TraceLayer};

use ggater::{GGater, StatusMap};

type TypedGGater = GGater<ggater::SqlitePhotoDatabase>;

pub async fn router() {
    let app = Router::new()
        .nest("/api", backend())
        .fallback_service(frontend())
        .layer(TraceLayer::new_for_http())
        .into_make_service();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Serve the static web artifact directory
fn frontend() -> Router {
    Router::new().fallback_service(
        ServeDir::new("build")
            .precompressed_gzip()
            .precompressed_br(),
    )
}

/// Serve the backend api routes
fn backend() -> Router {
    let ggater = Arc::new(Mutex::new(TypedGGater::new(ggater::SqlitePhotoDatabase {})));

    Router::new()
        .route("/status", get(get_scan_status))
        .route("/photos", get(get_photos))
        .route("/scan", get(init_scan))
        .layer(Extension(ggater))
}

async fn init_scan(Extension(ggater): Extension<Arc<Mutex<TypedGGater>>>) {
    ggater.lock().unwrap().scan().unwrap()
}

async fn get_scan_status(Extension(ggater): Extension<Arc<Mutex<TypedGGater>>>) -> Json<StatusMap> {
    let status = ggater.lock().unwrap().get_status();
    Json(status)
}

#[derive(Deserialize)]
struct PhotosRequest {
    page: usize,
}

#[derive(Serialize)]
struct PhotosResponse {
    data: usize,
}

async fn get_photos(
    Extension(ggater): Extension<Arc<Mutex<TypedGGater>>>,
    Json(photos): Json<PhotosRequest>,
) -> Json<PhotosResponse> {
    Json(PhotosResponse { data: 0 })
}
