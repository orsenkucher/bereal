use std::fmt::Display;

use juniper::{
    graphql_object, EmptySubscription, FieldResult, GraphQLEnum, GraphQLInputObject, GraphQLObject,
    ScalarValue,
};
use uuid::Uuid;
use warp::{http::Response, Filter};

use crate::{
    models,
    storage::{self, Service},
};

// #[derive(GraphQLEnum)]
// enum Episode {
//     NewHope,
//     Empire,
//     Jedi,
// }

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct User {
    id: String,
    telegram_id: String,
    name: Option<String>,
    friends: Vec<User>,
}

struct Users(Vec<User>);

impl From<(models::User, Vec<models::User>)> for User {
    fn from((u, f): (models::User, Vec<models::User>)) -> Self {
        let f: Users = f.into();
        Self {
            id: u.id.to_string(),
            telegram_id: u.telegram_id,
            name: u.name,
            friends: f.0,
        }
    }
}

impl From<Vec<models::User>> for Users {
    fn from(from: Vec<models::User>) -> Self {
        Self(
            from.into_iter()
                .map(|u| (u, vec![]))
                .map(Into::into)
                .collect(),
        )
    }
}

// There is also a custom derive for mapping GraphQL input objects.
#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewUser {
    name: String,
    telegram_id: String,
    // appears_in: Vec<Episode>,
    // home_planet: String,
}

#[derive(GraphQLInputObject)]
struct AddFriend {
    user_id: String,
    friend_id: String,
}

struct Context {
    // Use your real database pool here.
    storage: Service,
}

impl Context {
    fn new(storage: Service) -> Self {
        Self { storage }
    }
}

// marker trait
impl juniper::Context for Context {}

struct Query;
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
        let users = db.with_friends(users)?;
        let graph = users.into_iter().map(Into::into).collect();
        Ok(graph)

        // let mut result = vec![];
        // for (u, f) in users{
        //     result.push(User{
        //          id:u.id,
        //         telegram_id:u.telegram_id,
        //          name:u.name,

        //     })
        // }

        // todo!()
        // let friends =

        // Ok(users)
        // Execute a db query.
        // Note the use of `?` to propagate errors.
        // let human = connection.find_human(&id)?;
        // Return the result.
    }
}

// Now, we do the same for our Mutation type.
struct Mutation;

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
        let NewUser { name, telegram_id } = new_user;
        let user = db.create_user(models::NewUser::joined_now(&name, &telegram_id))?;
        Ok((user, vec![]).into())
    }

    fn add_friend(context: &Context, new_friend: AddFriend) -> FieldResult<User> {
        let db = &context.storage;
        let AddFriend { user_id, friend_id } = new_friend;
        let result = db.create_friend(models::NewFriend {
            user_id: Uuid::parse_str(&user_id)?,
            friend_id: Uuid::parse_str(&friend_id)?,
        })?;
        let user = db.user_by_id(result.user_id)?;
        Ok((user, vec![]).into())
    }
}

// A root schema consists of a query, a mutation, and a subscription.
// Request queries can be executed against a RootNode.
type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

fn schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}

pub async fn run() {
    let log = warp::log("warp_server");

    let homepage = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(format!(
                "<html><h1>BeReal API</h1><div>visit <a href=\"/graphiql\">/graphiql</a></html>"
            ))
    });

    tracing::info!("listening on 127.0.0.1:8080");

    let state = warp::any().map(move || Context::new(storage::Service::from_env()));
    let graphql_filter = juniper_warp::make_graphql_filter(schema(), state.boxed());

    warp::serve(
        warp::get()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql", None))
            .or(homepage)
            .or(warp::path("graphql").and(graphql_filter))
            .with(log),
    )
    .run(([127, 0, 0, 1], 8080))
    .await
}
