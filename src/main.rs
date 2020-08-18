#![feature(proc_macro_hygiene, decl_macro)]

mod model;
mod schema;

use juniper::{RootNode};
use model::Database;
use rocket::response::content;
use rocket::State;
use schema::{MutationRoot, Query};
use mongodb::{Client, Collection};
use lazy_static::lazy_static;

type Schema = RootNode<'static, Query, MutationRoot>;

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}


lazy_static! {
    pub static ref MONGO: Client = create_mongo_client();
}


fn create_mongo_client() -> Client {
    Client::with_uri_str("mongodb+srv://username:password@cluster.mongodb.net")
    .expect("Failed to initialize standalone client.")

}



fn collection(coll_name: &str) -> Collection {
    MONGO.database("collection").collection(coll_name)
}

fn main() {
    rocket::ignite()
        .manage(Database::new())
        .manage(Schema::new(Query, MutationRoot))
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}
