use bereal::{
    migrations,
    storage::{establish_connection, Database},
    util, BoxError,
};
use dotenvy::dotenv;
use futures::join;

#[tokio::main]
async fn main() {
    run().await
}

async fn run() {
    dotenv().ok();

    util::setup_tracing();
    tracing::info!("BeReal is starting");

    run_migrations().expect("failed to run database migrations");

    let db = Database::from_env();
    let api_fut = start_api_server(db.clone());
    let tg_fut = start_telegram_server(db);
    join!(api_fut, tg_fut);
}

fn run_migrations() -> Result<(), BoxError> {
    let conn = &mut establish_connection();
    tracing::info!("running database migrations");
    migrations::run(conn)
}

async fn start_api_server(db: Database) {
    tracing::info!("starting API server");
    bereal::api::run(db).await;
}

async fn start_telegram_server(db: Database) {
    use bereal::bot;
    tracing::info!("starting Telegram server");
    let tg_schema = bot::schema::root();
    let bot = bot::bot_from_env();
    bot::dispatch(bot, tg_schema, db).await;
}
