use std::env;
use std::ops::DerefMut;

use anyhow::{Context, Result};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;

use crate::models::{Friend, NewFriend};
use crate::{
    models::{NewUser, User},
    with_id::WithId,
};

pub type Connection = PgConnection;
pub type Pool<C> = r2d2::Pool<ConnectionManager<C>>;
type PooledConnection = r2d2::PooledConnection<ConnectionManager<Connection>>;

pub struct Service {
    pool: Pool<Connection>,
}

impl Service {
    pub fn new<S: Into<String>>(db_url: S) -> Self {
        let manager = ConnectionManager::new(db_url);
        let pool = Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .expect("could not build connection pool");
        Self { pool }
    }

    pub fn from_env() -> Self {
        let db_url = database_url();
        Self::new(db_url)
    }

    pub fn pool(&self) -> &Pool<Connection> {
        &self.pool
    }

    fn conn(&self) -> Result<PooledConnection> {
        self.pool
            .get()
            .with_context(|| "failed to get a connection from pool".to_owned())
    }

    pub fn create_user(&mut self, new_user: NewUser) -> Result<User> {
        create_user(new_user, self.conn()?.deref_mut())
    }

    pub fn create_friend(&mut self, new_friend: NewFriend) -> Result<Friend> {
        create_friend(new_friend, self.conn()?.deref_mut())
    }

    pub fn get_friends_for_user(&mut self, user: &User) -> Result<Vec<User>> {
        get_friends_for_user(user, self.conn()?.deref_mut())
    }

    pub(crate) fn reset_db(&mut self) {
        reset_db(self.conn().unwrap().deref_mut())
    }
}

fn database_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must bet set")
}

pub fn establish_connection() -> Connection {
    use diesel::Connection;
    let db_url = database_url();
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("error connecting to {db_url}"))
}

fn reset_db(conn: &mut Connection) {
    use crate::schema::friends::dsl::*;
    use crate::schema::users::dsl::*;
    diesel::delete(users)
        .execute(conn)
        .expect("could not delete users");
    diesel::delete(friends)
        .execute(conn)
        .expect("could not delete friends");
}

fn create_user(new_user: NewUser, conn: &mut Connection) -> Result<User> {
    use crate::schema::users::dsl::*;
    diesel::insert_into(users)
        .values(new_user.with_id())
        .get_result(conn)
        .with_context(|| format!("failed to create new user: {new_user:?}"))
}

fn create_friend(new_friend: NewFriend, conn: &mut Connection) -> Result<Friend> {
    use crate::schema::friends::dsl::*;
    diesel::insert_into(friends)
        .values(new_friend.with_id())
        .get_result(conn)
        .with_context(|| format!("failed to create new friend: {new_friend:?}"))
}

fn get_friends_for_user(user: &User, conn: &mut Connection) -> Result<Vec<User>> {
    use crate::schema::{friends, users};
    let friend_ids = Friend::belonging_to(user).select(friends::friend_id);
    users::table
        .filter(users::id.eq_any(friend_ids))
        .load::<User>(conn)
        .with_context(|| format!("failed to get friends for user: {user:?}"))
}
