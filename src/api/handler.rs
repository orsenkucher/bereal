use std::convert::Infallible;

use anyhow::Result;
use warp::{reply, Reply};

use crate::models::{Friend, User};
use crate::Database;

use super::models::{AddFriend, ListOptions, NewUser};
use super::{warp_try, ReplyError};

pub async fn list_users(opts: ListOptions, db: Database) -> Result<impl Reply, Infallible> {
    let users = list_users_inner(opts, db).await;
    let users = users
        .map(|users| reply::json(&users))
        .map_err(ReplyError::DatabaseError);
    warp_try!(users)
}

async fn list_users_inner(
    ListOptions { offset, limit }: ListOptions,
    db: Database,
) -> Result<Vec<User>> {
    let users = db.users_range(offset, limit)?;
    Ok(users)
}

pub async fn create_user(new_user: NewUser, db: Database) -> Result<impl Reply, Infallible> {
    let user = create_user_inner(&new_user, db).await;
    let user = user
        .map(|user| reply::json(&user))
        .map_err(ReplyError::DatabaseError);
    warp_try!(user)
}

async fn create_user_inner(new_user: &NewUser, db: Database) -> Result<User> {
    let user = db.create_user(new_user.into())?;
    Ok(user)
}

pub async fn add_friend(add_friend: AddFriend, db: Database) -> Result<impl Reply, Infallible> {
    let friend = add_friend_inner(add_friend, db).await;
    let friend = friend
        .map(|friend| reply::json(&friend))
        .map_err(ReplyError::DatabaseError);
    warp_try!(friend)
}

async fn add_friend_inner(add_friend: AddFriend, db: Database) -> Result<Friend> {
    let friend = db.create_friend(add_friend.as_uuid()?)?;
    Ok(friend)
}
