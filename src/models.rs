use chrono::NaiveDateTime;
use diesel::prelude::*;
use juniper::{graphql_object, FieldResult};
use serde::Deserialize;
use uuid::Uuid;

use crate::api::Context;
use crate::schema::*;
use crate::with_id::WithId;

#[derive(Identifiable, Queryable, AsChangeset, Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub telegram_id: String,
    pub name: Option<String>,
    pub joined_at: NaiveDateTime,
}

#[graphql_object(description = "Bereal application user")]
impl User {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn telegram_id(&self) -> &str {
        &self.telegram_id
    }

    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    fn joined_at(&self) -> &NaiveDateTime {
        &self.joined_at
    }

    fn friends(&self, context: &Context) -> FieldResult<Vec<User>> {
        let db = context.storage();
        let friends = db.friends_for_user(&self)?;
        Ok(friends)
    }
}

impl User {
    // normal block
}

#[derive(Insertable, Deserialize, Clone, Debug)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub telegram_id: &'a str,
    pub name: &'a str,
    pub joined_at: NaiveDateTime,
}

impl<'a> NewUser<'a> {
    pub fn joined_now(name: &'a str, telegram_id: &'a str) -> Self {
        let now = chrono::Utc::now();
        Self {
            telegram_id,
            name,
            joined_at: now.naive_utc(),
        }
    }
}

impl WithId for NewUser<'_> {
    type Id = users::id;

    fn id() -> Self::Id {
        users::id
    }
}

#[derive(Identifiable, Queryable, AsChangeset, Associations, Clone, Debug)]
#[diesel(belongs_to(User))]
// #[diesel(belongs_to(User, foreign_key = friend_id))]
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

impl NewFriend {
    pub fn inverse(&self) -> Self {
        Self {
            user_id: self.friend_id,
            friend_id: self.user_id,
        }
    }
}

impl WithId for NewFriend {
    type Id = friends::id;

    fn id() -> Self::Id {
        friends::id
    }
}
