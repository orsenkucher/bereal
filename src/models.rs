use diesel::prelude::*;

use crate::schema::posts;

#[derive(Queryable, Identifiable, AsChangeset)]
// #[table_name = "posts"]
// #[primary_key(id)]
// #[changeset_options(treat_none_as_null="true")]
// or Option<Option<T>> field
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub draft: bool,
    pub published_at: chrono::NaiveDateTime,
    pub visit_count: i32,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
