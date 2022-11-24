use bereal::{migrations, models::Post, storage::establish_connection, util};
use diesel::prelude::*;
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

    use bereal::schema::posts::dsl::*;
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(connection)
        .expect("error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("--------\n");
        println!("{}", post.body);
    }

    // Storage is repository wrapping diesel connection
    // let storage = bereal::storage().await;

    let schema = bereal::bot::schema::root();
    let bot = bereal::bot::bot_from_env();

    bereal::dispatch(bot, schema).await;
}
