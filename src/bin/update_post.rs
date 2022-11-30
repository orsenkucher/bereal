use bereal::models::Post;
use bereal::storage::establish_connection;

use diesel::{debug_query, pg::Pg, prelude::*};

fn main() {
    let connection = &mut establish_connection();
    update_post(connection).unwrap();
}

type DB = Pg;

pub fn update_post(conn: &mut PgConnection) -> anyhow::Result<Post> {
    use bereal::schema::posts::dsl::{draft, posts};

    // let sql = debug_query::<PgConnection, _>(diesel::update(posts.find(7))).set(published.eq(true));
    // let sql = debug_query::<Pg, _>(&posts.count());
    let binding = diesel::update(posts.find(7)).set(draft.eq(false));
    let sql = debug_query::<DB, _>(&binding);
    println!("{:?}", sql);

    let id = std::env::args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");

    let post = diesel::update(posts.find(id))
        .set(draft.eq(false))
        .get_result::<Post>(conn)?;

    // diesel::update(posts)
    //     .set(published.eq(true))
    //     .execute(conn)?;

    Ok(post)
}
