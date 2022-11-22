use dotenvy::dotenv;
use tracing_subscriber::prelude::*;

pub fn setup_tracing() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,main=trace,bereal=trace".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
}
