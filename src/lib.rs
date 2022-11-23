use teloxide::{prelude::RequesterExt, types::ParseMode, Bot};

pub mod migrations;
pub mod models;
pub mod schema;
pub mod storage;
pub mod util;

// type MyBot = DefaultParseMode<Bot>;

// pub fn bot_from_env() -> MyBot {
//     Bot::from_env().parse_mode(ParseMode::Html).auto_send()
// }

// pub async fn storage(path: impl AsRef<str>) -> MyStorage {
//     Sqlite
// }

// pub fn dispatch();

// pub fn schema();

#[cfg(test)]
mod tests {
    struct MyType;

    fn is_normal<T: Sized + Send + Sync + Unpin>() {}

    #[test]
    fn normal_types() {
        is_normal::<MyType>();
    }
}

// pub enum CopyError {
//     In(std::io::Error),
//     Out(std::io::Error),
// }

// type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
