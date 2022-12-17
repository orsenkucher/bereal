use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use crate::schema::*;
use crate::with_id::WithId;

#[derive(Identifiable, Queryable, AsChangeset, Clone, Debug)]
pub struct User {
    id: Uuid,
    pub telegram_id: String,
    // name: Option<String>,
    pub name: String,
    pub joined_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Clone, Debug)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub telegram_id: &'a str,
    pub name: &'a str,
    pub joined_at: NaiveDateTime,
}

impl WithId for NewUser<'_> {
    type Id = users::id;

    fn id() -> Self::Id {
        users::id
    }
}

#[derive(Identifiable, Queryable, AsChangeset, Associations, Clone, Debug)]
#[diesel(belongs_to(User))]
#[diesel(table_name = friends)]
pub struct Friend {
    pub id: Uuid,
    pub user_id: Uuid,
    pub friend_id: Uuid,
}

#[derive(Insertable, Deserialize, Clone, Debug)]
#[diesel(table_name = friends)]
pub struct NewFriend {
    pub user_id: Uuid,
    pub friend_id: Uuid,
}

impl WithId for NewFriend {
    type Id = friends::id;

    fn id() -> Self::Id {
        friends::id
    }
}
