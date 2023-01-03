use std::error::Error;

pub mod api;
pub mod bot;
pub mod migrations;
pub mod models;
pub mod schema;
pub mod storage;
pub mod util;
mod with_id;

pub type BoxError = Box<dyn Error + Send + Sync + 'static>;

#[cfg(test)]
mod tests {
    struct MyType;

    fn is_normal<T: Sized + Send + Sync + Unpin>() {}

    #[test]
    fn normal_types() {
        is_normal::<MyType>();
    }
}
