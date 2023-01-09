use serde::{Deserialize, Serialize};

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

impl<'a> From<&'a NewUser> for crate::models::NewUser<'a> {
    fn from(
        NewUser {
            phone_number,
            chat_id,
        }: &'a NewUser,
    ) -> Self {
        Self::joined_now(phone_number, chat_id)
    }
}
