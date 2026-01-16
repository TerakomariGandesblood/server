use std::net::{IpAddr, SocketAddr};

use anyhow::Result;
use clap::Parser;
use mimalloc::MiMalloc;
use server::Args;
use tokio::net::TcpListener;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let _guard = server::init_log(&args.verbose, "log", env!("CARGO_CRATE_NAME"));

    let router = server::router();
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(args.host), args.port)).await?;
    tracing::info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, router)
        .with_graceful_shutdown(server::shutdown_signal())
        .await?;

    Ok(())
}
