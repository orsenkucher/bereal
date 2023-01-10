use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Debug, Deserialize)]
pub struct ListOptions {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewUser {
    pub chat_id: String,
    pub phone_number: String,
}

impl<'a> From<&'a NewUser> for models::NewUser<'a> {
    fn from(
        NewUser {
            phone_number,
            chat_id,
        }: &'a NewUser,
    ) -> Self {
        Self::joined_now(phone_number, chat_id)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AddFriend {
    pub user_id: String,
    pub friend_id: String,
}

impl AddFriend {
    pub fn as_uuid(&self) -> Result<models::NewFriend> {
        models::NewFriend::from_str(&self.user_id, &self.friend_id)
    }
}
