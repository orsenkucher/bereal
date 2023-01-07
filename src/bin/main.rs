use dotenvy::dotenv;

use bereal::database::Database;
use bereal::util;

#[tokio::main]
async fn main() {
    run().await
}

async fn run() {
    dotenv().ok();

    util::setup_tracing();
    tracing::info!("BeReal is starting");

    bereal::run_migrations().expect("failed to run database migrations");

    let db = Database::from_env();
    bereal::start(db).await;
}
