use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, StatusCode},
    Router,
};
use reqwest::Client;
use std::sync::Arc;
use axum::body::to_bytes;
use axum::routing::any;
use log::{error, info};
use crate::config::TargetServiceConfig;

#[derive(Clone)]
struct AppState {
    client: Arc<Client>,
    target_host: String,
    target_port: u16,
}

async fn proxy_request(
    State(state): State<AppState>,
    req: Request<Body>,
) -> Result<Response<Body>, StatusCode> {
    let target_uri = format!(
        "http://{}:{}{}",
        state.target_host,
        state.target_port,
        req.uri()
    );

    let method = req.method().clone();
    let headers = req.headers().clone();

    let body_bytes = to_bytes(req.into_body(), 0)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    info!("Sending request to target: {}", target_uri);

    let client = &state.client;
    let mut request_builder = client.request(method, &target_uri);

    for (key, value) in headers.iter() {
        request_builder = request_builder.header(key, value);
    }

    let response = request_builder
        .body(body_bytes)
        .send()
        .await
        .map_err(|err| {
            error!("Request to target failed: {}", err);
            StatusCode::BAD_GATEWAY
        })?;
    info!(
        "Received response from target: {} - Status: {}",
        target_uri,
        response.status()
    );

    let mut builder = Response::builder().status(response.status());
    for (key, value) in response.headers() {
        builder = builder.header(key, value);
    }

    let body = response.bytes().await.map_err(|_| StatusCode::BAD_GATEWAY)?;

    Ok(builder.body(Body::from(body)).unwrap())
}

pub fn app_router(config: TargetServiceConfig) -> Router {
    let client = Arc::new(Client::new());
    let state = AppState {
        client,
        target_host: config.host,
        target_port: config.port,
    };

    Router::new()
        .route("/*path", any(proxy_request))
        .with_state(state)
}
