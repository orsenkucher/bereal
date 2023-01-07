use std::error::Error;

use futures::join;

use database::Database;

mod api;
mod bot;
pub mod database;
mod graphql;
mod migrations;
mod models;
mod schema;
pub mod util;
mod with_id;

pub type BoxError = Box<dyn Error + Send + Sync + 'static>;

pub fn run_migrations() -> Result<(), BoxError> {
    let conn = &mut database::establish_connection();
    tracing::info!("running database migrations");
    migrations::run(conn)
}

pub async fn start(db: Database) {
    let api_fut = start_api_server(db.clone());
    let tg_fut = start_telegram_server(db);
    join!(api_fut, tg_fut);
}

async fn start_api_server(db: Database) {
    tracing::info!("starting API server");
    api::run(db).await;
}

async fn start_telegram_server(db: Database) {
    tracing::info!("starting Telegram server");
    let tg_schema = bot::schema();
    let bot = bot::bot_from_env();
    bot::dispatch(bot, tg_schema, db).await;
}

#[cfg(test)]
mod tests {
    struct MyType;

    fn is_normal<T: Sized + Send + Sync + Unpin>() {}

    #[test]
    fn normal_types() {
        is_normal::<MyType>();
    }
}
