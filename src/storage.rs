use std::env;
use std::ops::DerefMut;

use anyhow::{Context, Result};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
use uuid::Uuid;

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

    pub fn create_user(&self, new_user: NewUser) -> Result<User> {
        create_user(new_user, self.conn()?.deref_mut())
    }

    pub fn create_friend(&self, new_friend: NewFriend) -> Result<Friend> {
        create_friend(new_friend, self.conn()?.deref_mut())
    }

    pub fn user_by_id(&self, id: Uuid) -> Result<User> {
        get_user_by_id(id, self.conn()?.deref_mut())
    }

    pub fn friends_for_user(&self, user: &User) -> Result<Vec<User>> {
        get_friends_for_user(user, self.conn()?.deref_mut())
    }

    pub fn users(&self) -> Result<Vec<User>> {
        get_users(self.conn()?.deref_mut())
    }

    pub fn with_friends(&self, users: Vec<User>) -> Result<Vec<(User, Vec<User>)>> {
        get_friends_for_users(users, self.conn()?.deref_mut())
    }

    pub(crate) fn reset_db(&self) {
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

fn get_user_by_id(id: Uuid, conn: &mut Connection) -> Result<User> {
    use crate::schema::users::dsl::*;
    users
        .find(id)
        .first(conn)
        .with_context(|| format!("failed to get user by id: {id:?}"))
}

fn get_users(conn: &mut Connection) -> Result<Vec<User>> {
    use crate::schema::users::dsl::*;
    users
        .load(conn)
        .with_context(|| "failed to get users".to_owned())
}

fn get_users_by_ids(ids: &[Uuid], conn: &mut Connection) -> Result<Vec<User>> {
    use crate::schema::users::dsl::*;
    users
        .filter(id.eq_any(ids))
        .load(conn)
        .with_context(|| format!("failed to get users by ids, len: {}", ids.len()))
}

fn get_friends_for_users(
    users: Vec<User>,
    conn: &mut Connection,
) -> Result<Vec<(User, Vec<User>)>> {
    let friends = Friend::belonging_to(&users)
        .load::<Friend>(conn)?
        .grouped_by(&users);
    let ids = friends
        .concat()
        .into_iter()
        .map(|f| f.friend_id)
        .collect::<Vec<_>>();
    let friend_users = get_users_by_ids(&ids, conn)?;
    let friends = shape(friend_users, &friends);
    let data = users.into_iter().zip(friends).collect::<Vec<_>>();
    Ok(data)
}

fn shape<T, U>(target: Vec<T>, mask: &[Vec<U>]) -> Vec<Vec<T>> {
    let mask = mask.iter().map(|x| x.len()).collect::<Vec<_>>();
    chunk_by(target, &mask)
}

fn chunk_by<T>(mut target: Vec<T>, mask: &[usize]) -> Vec<Vec<T>> {
    use std::mem::swap;
    mask.iter()
        .map(|&size| {
            let mut right = target.split_off(size);
            swap(&mut target, &mut right);
            right
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{chunk_by, shape};

    #[test]
    fn test_shape() {
        let target: Vec<i32> = (0..10).collect();
        let mask = vec![vec![(); 2], vec![(); 3], vec![(); 5]];
        let result = shape(target, &mask);
        assert_eq!(
            result,
            vec![vec![0, 1], vec![2, 3, 4], vec![5, 6, 7, 8, 9],]
        );
    }

    #[test]
    fn test_chunk_by() {
        let target: Vec<i32> = (0..10).collect();
        let mask = [0, 4, 2, 3, 1];
        let result = chunk_by(target, &mask);
        assert_eq!(
            result,
            vec![vec![], vec![0, 1, 2, 3], vec![4, 5], vec![6, 7, 8], vec![9]]
        );
    }
}
