use dotenvy::dotenv;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::{prelude::*, EnvFilter};

pub fn setup_tracing() {
    dotenv().ok();

    let filter = EnvFilter::new(
        std::env::var("RUST_LOG")
            .unwrap_or_else(|_| "info,main=trace,bereal=trace,warp=debug".into()),
    );

    let fmt_layer = tracing_subscriber::fmt::layer().with_span_events(FmtSpan::CLOSE);

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .init();
}
