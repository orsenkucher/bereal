use bereal::{migrations, models::Post, storage::establish_connection, util};
use diesel::prelude::*;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
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
