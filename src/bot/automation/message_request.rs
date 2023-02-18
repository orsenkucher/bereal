use teloxide::payloads::{EditMessageText, SendMessage};
use teloxide::requests::JsonRequest;
use teloxide::types::MessageId;

pub type Request<T> = JsonRequest<T>;

pub trait MessageRequest {
    fn send(self) -> Request<SendMessage>;
    fn edit(self, msg_id: MessageId) -> Option<Request<EditMessageText>>;
}
