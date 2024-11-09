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
use tokio_util::bytes;
use bytes::Bytes;
use tracing::{Level, span};
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

    // put sidecar logic here
    let method = req.method().clone();
    let span = span!(Level::INFO, "proxy_request", method = %method, uri = %target_uri);
    info!("proxy_request: method={}, uri={}", method, target_uri);
    let _enter = span.enter();

    match forward_request(&state.client, req, &target_uri).await {
        Ok(response) => {
            info!("Response received with status: {}", response.status());
            Ok(response)
        }
        Err(err) => {
            error!("Failed to proxy request: {}", err);
            Err(StatusCode::BAD_GATEWAY)
        }
    }
}

async fn forward_request(
    client: &Client,
    req: Request<Body>,
    target_uri: &str,
) -> Result<Response<Body>, reqwest::Error> {
    let method = req.method().clone();
    let headers = req.headers().clone();

    let body_bytes = to_bytes(req.into_body(), 0).await.unwrap_or_else(|err| {
        error!("Failed to read request body: {}", err);
        Bytes::new()
    });

    let mut request_builder = client.request(method, target_uri).body(body_bytes);

    for (key, value) in headers {
        if let (Some(key), Some(value)) = (key, value.to_str().ok()) {
            request_builder = request_builder.header(key, value);
        }
    }

    let response = request_builder.send().await?;
    let status = response.status();
    let headers = response.headers().clone();
    let body = response.bytes().await?;

    let mut builder = Response::builder().status(status);
    for (key, value) in headers {
        if let (Some(key), Some(value)) = (key, value.to_str().ok()) {
            builder = builder.header(key, value);
        }
    }
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
