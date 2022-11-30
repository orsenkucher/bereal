use bereal::{
    models::{NewPost, Post},
    storage::establish_connection,
};

use diesel::prelude::*;

fn main() {
    let connection = &mut establish_connection();

    let title = "Hello world";
    let body = "This is my post";
    let post = create_post(connection, title, body);
    create_posts(connection, &[], &[]).unwrap();

    tracing::info!("Saved draft {} with id {}", post.title, post.id);
    show_posts(connection);
}

fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {
    use bereal::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("error saving new post")
}

fn create_posts(
    conn: &mut PgConnection,
    titles: &[&str],
    bodies: &[&str],
) -> anyhow::Result<Vec<Post>> {
    use bereal::schema::posts;

    let new_posts: Vec<_> = titles
        .iter()
        .zip(bodies.iter())
        .map(|(title, body)| NewPost { title, body })
        .collect();

    let results = diesel::insert_into(posts::table)
        .values(new_posts)
        .get_results(conn)?;

    Ok(results)
}

fn show_posts(connection: &mut PgConnection) {
    use bereal::schema::posts::dsl::*;
    let results = posts
        .filter(draft.eq(false))
        .limit(5)
        .load::<Post>(connection)
        .expect("error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("--------\n");
        println!("{}", post.body);
    }
}
