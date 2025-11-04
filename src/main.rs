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

    if args.install {
        server::install_start_service(env!("CARGO_CRATE_NAME"), &args.path, &args.verbose)?;
        return Ok(());
    }
    if args.uninstall {
        server::stop_uninstall_service(env!("CARGO_CRATE_NAME"))?;
        return Ok(());
    }

    let router = server::router(&args.path);
    let listener = TcpListener::bind("127.0.0.1:8001").await?;
    tracing::info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, router)
        .with_graceful_shutdown(server::shutdown_signal())
        .await?;

    Ok(())
}
