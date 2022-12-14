use std::error::Error;

use bot::MyBot;
use diesel::expression::AsExpression;
use teloxide::prelude::*;

pub mod bot;
pub mod migrations;
pub mod models;
pub mod schema;
pub mod storage;
pub mod util;

type BoxError = Box<dyn Error + Send + Sync + 'static>;
type HandlerResult = Result<(), BoxError>;

type MyHandler = dptree::Handler<
    'static,
    DependencyMap,
    HandlerResult,
    teloxide::dispatching::DpHandlerDescription,
>;

pub async fn dispatch(bot: MyBot, schema: MyHandler) {
    Dispatcher::builder(bot, schema)
        .dependencies(dptree::deps![])
        .default_handler(|upd| async move {
            tracing::warn!("unhandled update: {upd:?}");
        })
        .error_handler(LoggingErrorHandler::with_custom_text(
            "an error has occured in the dispatcher",
        ))
        .build()
        .dispatch()
        .await;
}

use diesel::prelude::*;
fn update_indepth(conn: &mut PgConnection) -> anyhow::Result<usize> {
    use crate::schema::posts::dsl::*;
    use diesel::dsl::now;

    let result = diesel::update(posts)
        .filter(published_at.lt(now))
        .set(draft.eq(false))
        .execute(conn)?;

    Ok(result)
}

use diesel::dsl::Eq;
use diesel::sql_types::Text;
sql_function!(fn canon_crate_name(x: Text) -> Text);

type WithName<T> = Eq<canon_crate_name::HelperType<creates::name>, canon_crate_name::HelperType<T>>;

fn with_name<T>(name: T) -> WithName<T>
where
    T: AsExpression<Text>,
{
    canon_crate_name(crates::name).eq(canon_crate_name(name))
}
// crates::table.filter(with_name("foo"))

fn with_name<'a, T>(name: T) -> Box<BoxableExpression<crates::table, Pg, SqlType = Bool> + 'a>
where
    T: AsExpression<Text>,
    T::Expression: BoxableExpression<crates::table, Pg>,
{
    canon_crate_name(crates::name).eq(canon_crate_name(name))
}

#[cfg(test)]
mod tests {
    struct MyType;

    fn is_normal<T: Sized + Send + Sync + Unpin>() {}

    #[test]
    fn normal_types() {
        is_normal::<MyType>();
    }
}
