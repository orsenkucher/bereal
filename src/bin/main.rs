use bereal::{migrations, storage::establish_connection, util};
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    run().await
}

async fn run() {
    dotenv().ok();

    util::setup_tracing();
    tracing::info!("BeReal is starting");

    let connection = &mut establish_connection();
    migrations::run(connection).unwrap();

    bereal::api::run().await;

    // Storage is repository wrapping diesel connection
    // let storage = bereal::storage().await;

    // let schema = bereal::bot::schema::root();
    // let bot = bereal::bot::bot_from_env();

    // bereal::dispatch(bot, schema).await;
}
