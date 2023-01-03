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
    pub chat_id: String,
    pub phone_number: Option<String>,
    pub joined_at: NaiveDateTime,
}

#[graphql_object(description = "Bereal application user")]
impl User {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn chat_id(&self) -> &str {
        &self.chat_id
    }

    fn phone_number(&self) -> Option<&str> {
        self.phone_number.as_deref()
    }

    fn joined_at(&self) -> &NaiveDateTime {
        &self.joined_at
    }

    fn friends(&self, context: &Context) -> FieldResult<Vec<User>> {
        let db = context.storage();
        let friends = db.friends_for_user(self)?;
        Ok(friends)
    }
}

impl User {
    pub fn is_registered(&self) -> bool {
        self.phone_number.is_some()
    }
}

#[derive(Insertable, Deserialize, Clone, Debug)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub chat_id: &'a str,
    pub phone_number: &'a str,
    pub joined_at: NaiveDateTime,
}

impl<'a> NewUser<'a> {
    pub fn joined_now(phone_number: &'a str, chat_id: &'a str) -> Self {
        let now = chrono::Utc::now();
        Self {
            chat_id,
            phone_number,
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
