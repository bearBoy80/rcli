use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use tracing::{error, info};
#[derive(Debug)]
struct AppState {
    path: PathBuf,
}
pub async fn http_server(path: PathBuf, port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("http server:{}", addr);
    let state = AppState { path };

    let app = Router::new()
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn file_handler(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let full_path = std::path::Path::new(&state.path).join(path);
    if !full_path.exists() {
        (StatusCode::NOT_FOUND, String::from("not found"))
    } else {
        match tokio::fs::read_to_string(&full_path).await {
            Ok(content) => (StatusCode::OK, content),
            Err(e) => {
                error!("Failed to read:{:?}", full_path);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}
