use std::{fs, io, net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{
    body::Body,
    extract::{Path, State},
    http::{header::CONTENT_TYPE, StatusCode},
    response::Response,
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use tracing::{error, info};
#[derive(Debug)]
struct AppState {
    path: PathBuf,
}
pub async fn http_server(path: PathBuf, port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("http server:{}", addr);
    let state = AppState { path: path.clone() };

    let app = Router::new()
        .route("/*path", get(file_handler_response))
        .nest_service("/tower", ServeDir::new(path))
        .with_state(Arc::new(state));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
#[allow(dead_code)]
async fn file_handler(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let full_path = std::path::Path::new(&state.path).join(path);
    if !full_path.exists() {
        (StatusCode::NOT_FOUND, String::from("not found"))
    } else {
        //如果是目录的话，获取目录下的所有文件
        if full_path.is_dir() {
            let mut entries = fs::read_dir(full_path)
                .unwrap()
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, io::Error>>()
                .unwrap();
            entries.sort();
            let string_paths: Vec<String> = entries
                .into_iter()
                .map(|p| format!("<li><a href={}/>{}</li>", p.display(), p.display()))
                .collect();
            let content = format!(
                "<html><head><title>list file</title></head><body>{}</body</html>",
                string_paths.join("")
            );
            return (StatusCode::OK, content);
        }
        match tokio::fs::read_to_string(&full_path).await {
            Ok(content) => (StatusCode::OK, content),
            Err(e) => {
                error!("Failed to read:{:?}", full_path);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}
async fn file_handler_response(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> axum::response::Response {
    let full_path = std::path::Path::new(&state.path).join(path);
    let response = Response::builder();
    if !full_path.exists() {
        response
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("file not found"))
            .unwrap()
    } else {
        //如果是目录的话，获取目录下的所有文件
        if full_path.is_dir() {
            let mut entries = fs::read_dir(full_path)
                .unwrap()
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, io::Error>>()
                .unwrap();
            entries.sort();
            let string_paths: Vec<String> = entries
                .into_iter()
                .map(|p| format!("<li><a href={} />{}</li>", p.display(), p.display()))
                .collect();
            let content = format!(
                "<html><head><title>list file</title></head><body>{}</body</html>",
                string_paths.join("")
            );
            return response
                .status(StatusCode::OK)
                .header(CONTENT_TYPE, "text/html")
                .body(Body::from(content))
                .unwrap();
        }
        match tokio::fs::read_to_string(&full_path).await {
            Ok(content) => response
                .status(StatusCode::OK)
                .body(Body::from(content))
                .unwrap(),
            Err(e) => {
                error!("Failed to read:{:?}", full_path);
                response
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(e.to_string()))
                    .unwrap()
            }
        }
    }
}
