use std::env;

use anyhow::{Context, Result};
use diesel::prelude::*;
use dotenvy::dotenv;

use crate::models::{Friend, NewFriend};
use crate::{
    models::{NewUser, User},
    with_id::WithId,
};

pub struct Service {
    conn: PgConnection,
}

impl Service {
    pub fn new(conn: PgConnection) -> Self {
        Self { conn }
    }

    pub fn from_env() -> Self {
        let conn = establish_connection();
        Self::new(conn)
    }

    fn conn_mut(&mut self) -> &mut PgConnection {
        &mut self.conn
    }

    pub fn create_user(&mut self, new_user: NewUser) -> Result<User> {
        create_user(new_user, self.conn_mut())
    }

    pub fn create_friend(&mut self, new_friend: NewFriend) -> Result<Friend> {
        create_friend(new_friend, self.conn_mut())
    }

    pub fn get_friends_for_user(&mut self, user: &User) -> Result<Vec<User>> {
        get_friends_for_user(user, self.conn_mut())
    }

    pub(crate) fn reset_db(&mut self) {
        reset_db(self.conn_mut())
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must bet set");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("error connecting to {db_url}"))
}

fn reset_db(conn: &mut PgConnection) {
    use crate::schema::friends::dsl::*;
    use crate::schema::users::dsl::*;
    diesel::delete(users)
        .execute(conn)
        .expect("could not delete users");
    diesel::delete(friends)
        .execute(conn)
        .expect("could not delete friends");
}

fn create_user(new_user: NewUser, conn: &mut PgConnection) -> Result<User> {
    use crate::schema::users::dsl::*;
    diesel::insert_into(users)
        .values(new_user.with_id())
        .get_result(conn)
        .with_context(|| format!("failed to create new user: {new_user:?}"))
}

fn create_friend(new_friend: NewFriend, conn: &mut PgConnection) -> Result<Friend> {
    use crate::schema::friends::dsl::*;
    diesel::insert_into(friends)
        .values(new_friend.with_id())
        .get_result(conn)
        .with_context(|| format!("failed to create new friend: {new_friend:?}"))
}

fn get_friends_for_user(user: &User, conn: &mut PgConnection) -> Result<Vec<User>> {
    use crate::schema::{friends, users};
    let friend_ids = Friend::belonging_to(user).select(friends::friend_id);
    users::table
        .filter(users::id.eq_any(friend_ids))
        .load::<User>(conn)
        .with_context(|| format!("failed to get friends for user: {user:?}"))
}
