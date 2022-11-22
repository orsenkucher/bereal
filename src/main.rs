use bereal::{models::Post, storage::establish_connection, util};
use diesel::prelude::*;
use dotenvy::dotenv;

// TODO:
// 1. schema for bot message handling
// 2. diesel database setup
#[tokio::main]
async fn main() {
    dotenv().ok();

    util::setup_tracing();
    tracing::info!("BeReal is starting");

    use bereal::schema::posts::dsl::*;
    let connection = &mut establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("--------\n");
        println!("{}", post.body);
    }

    // let storage = bereal::storage().await;

    // let bot = bereal::bot();
    // let schema = bereal::schema();

    // bereal::dispatch(schema, bot, storage).await;
}
