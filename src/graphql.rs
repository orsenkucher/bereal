use std::fmt::Display;

use juniper::{graphql_object, EmptySubscription, FieldResult, GraphQLInputObject, ScalarValue};
use uuid::Uuid;

use crate::models::{self, User};
use crate::Database;

#[derive(GraphQLInputObject)]
#[graphql(description = "User creation input object.")]
struct NewUser {
    name: String,
    chat_id: String,
}

#[derive(GraphQLInputObject)]
struct AddFriend {
    user_id: String,
    friend_id: String,
}

pub struct Context {
    storage: Database,
}

impl Context {
    pub fn new(storage: Database) -> Self {
        Self { storage }
    }

    pub fn storage(&self) -> &Database {
        &self.storage
    }
}

// marker trait
impl juniper::Context for Context {}

pub struct Query;
#[graphql_object(
    // Here we specify the context type for the object.
    // We need to do this in every type that
    // needs access to the context.
    context = Context,
)]
impl Query {
    fn api_version() -> &'static str {
        "1.0"
    }

    // Arguments to resolvers can either be simple types or input objects.
    // To gain access to the context, we specify a argument
    // that is a reference to the Context type.
    // Juniper automatically injects the correct context here.
    // fn user(context: &Context, id: String) -> FieldResult<User> {
    //     // todo!()
    //     // Get a db connection.
    //     let connection = context.service.get_connection()?;
    //     // Execute a db query.
    //     // Note the use of `?` to propagate errors.
    //     let human = connection.find_human(&id)?;
    //     // Return the result.
    //     Ok(human)
    // }

    fn users(context: &Context) -> FieldResult<Vec<User>> {
        let db = &context.storage;
        let users = db.users()?;
        Ok(users)
    }
}

pub struct Mutation;

#[graphql_object(
    context = Context,
    // If we need to use `ScalarValue` parametrization explicitly somewhere
    // in the object definition (like here in `FieldResult`), we could
    // declare an explicit type parameter for that, and specify it.
    scalar = S: ScalarValue + Display,
)]
impl Mutation {
    fn create_user<S: ScalarValue + Display>(
        context: &Context,
        new_user: NewUser,
    ) -> FieldResult<User, S> {
        let db = &context.storage;
        let NewUser { name, chat_id } = new_user;
        let user = db.create_user(models::NewUser::joined_now(&name, &chat_id))?;
        Ok(user)
    }

    fn add_friend(context: &Context, new_friend: AddFriend) -> FieldResult<User> {
        let db = &context.storage;
        let AddFriend { user_id, friend_id } = new_friend;
        let result = db.create_friend(models::NewFriend {
            user_id: Uuid::parse_str(&user_id)?,
            friend_id: Uuid::parse_str(&friend_id)?,
        })?;
        let user = db.user_by_id(result.user_id)?;
        Ok(user)
    }
}

// A root schema consists of a query, a mutation, and a subscription.
// Request queries can be executed against a RootNode.
pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}
