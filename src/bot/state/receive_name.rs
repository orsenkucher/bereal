use serde::{Deserialize, Serialize};

use crate::bot::language::{self, Language};

use super::{StateMessage, StateResend};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReceiveName {
    pub lang: Language,
}

pub type Payload = (Language,);

impl StateMessage for ReceiveName {
    type Payload = Payload;

    fn message_text(&self, payload: Self::Payload) -> String {
        language::receive_name(&payload.0)
    }
}

impl StateResend for ReceiveName {
    fn resend_text(&self) -> String {
        language::receive_name_resend(&self.lang)
    }
}
