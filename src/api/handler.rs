use std::convert::Infallible;

use anyhow::Result;
use warp::{reply, Reply};

use crate::models::User;
use crate::Database;

use super::models::{ListOptions, NewUser};
use super::{warp_try, ReplyError};

pub async fn list_users(opts: ListOptions, db: Database) -> Result<impl Reply, Infallible> {
    let users = list_users_inner(opts, db).await;
    let users = users
        .map(|users| reply::json(&users))
        .map_err(ReplyError::DatabaseError);
    warp_try!(users)
}

async fn list_users_inner(opts: ListOptions, db: Database) -> Result<Vec<User>> {
    let users = db.users_range(opts.offset, opts.limit)?;
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
