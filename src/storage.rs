use std::env;

use diesel::prelude::*;
use dotenvy::dotenv;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must bet set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("error connecting to {database_url}"))
}
