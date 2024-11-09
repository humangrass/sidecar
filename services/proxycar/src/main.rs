mod cli;
mod config;
mod proxy;

use std::path::Path;
use crate::cli::Cli;
use crate::proxy::app_router;

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("Fatal error occurred: {}", err);
        std::process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    let cli = Cli::new();
    let config = config::SidecarConfig::new(Path::new(&cli.config)).expect("Failed to load config");

    let app = app_router(config.http.target_service);
    let address = format!("localhost:{}", config.http.listen_port);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
