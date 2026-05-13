use anyhow::Context;
use clap::Parser;
use kvs_ecosystem::{Store, handle_json};
use tracing::info;

#[derive(Debug, Parser)]
struct Args {
    #[arg(long)]
    request: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "kvs_ecosystem=info".to_string()),
        )
        .init();

    let args = Args::parse();
    let mut store = Store::default();
    let response = handle_json(&mut store, &args.request).context("handling request")?;

    info!(bytes = response.len(), "response generated");
    println!("{}", response);

    Ok(())
}
