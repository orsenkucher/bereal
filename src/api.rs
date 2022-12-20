use juniper::{
    graphql_object, EmptySubscription, FieldResult, GraphQLEnum, GraphQLInputObject, GraphQLObject,
};
use warp::{http::Response, Filter};

use crate::storage::{self, Service};

#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

// There is also a custom derive for mapping GraphQL input objects.
#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
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
    fn apiVersion() -> &'static str {
        "1.0"
    }

    // Arguments to resolvers can either be simple types or input objects.
    // To gain access to the context, we specify a argument
    // that is a reference to the Context type.
    // Juniper automatically injects the correct context here.
    fn human(context: &Context, id: String) -> FieldResult<Human> {
        todo!()
        // // Get a db connection.
        // let connection = context.pool.get_connection()?;
        // // Execute a db query.
        // // Note the use of `?` to propagate errors.
        // let human = connection.find_human(&id)?;
        // // Return the result.
        // Ok(human)
    }
}

// Now, we do the same for our Mutation type.
struct Mutation;

#[graphql_object(
    context = Context,
    // If we need to use `ScalarValue` parametrization explicitly somewhere
    // in the object definition (like here in `FieldResult`), we could
    // declare an explicit type parameter for that, and specify it.
    // scalar = S: ScalarValue + Display,
)]
impl Mutation {
    fn createHuman<S: ScalarValue + Display>(
        context: &Context,
        new_human: NewHuman,
    ) -> FieldResult<Human> {
        todo!()
    }
    // fn createHuman<S: ScalarValue + Display>(
    //     context: &Context,
    //     new_human: NewHuman,
    // ) -> FieldResult<Human, S> {
    //     let db = context
    //         .pool
    //         .get_connection()
    //         .map_err(|e| e.map_scalar_value())?;
    //     let human: Human = db
    //         .insert_human(&new_human)
    //         .map_err(|e| e.map_scalar_value())?;
    //     Ok(human)
    // }
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
                "<html><h1>juniper_warp</h1><div>visit <a href=\"/graphiql\">/graphiql</a></html>"
            ))
    });

    tracing::info!("Listening on 127.0.0.1:8080");

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
