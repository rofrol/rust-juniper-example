#[macro_use]
extern crate juniper;
extern crate serde_json;

mod model;
mod schema;

use juniper::http;
use juniper::RootNode;
use model::Database;
use juniper::EmptyMutation;

fn main() {
    let body = r#"
    {
        "query": "{ hero { name } }"
    }"#;

    let request: http::GraphQLRequest = serde_json::from_str(&body).unwrap();
    println!("{:?}", request); // GraphQLRequest { query: "{ hero { name } }", operation_name: None, variables: None }

    type Schema = RootNode<'static, Database, EmptyMutation<Database>>;
    let root_node = Schema::new(Database::new(), EmptyMutation::<Database>::new());
    let context = Database::new();
    let response = request.execute(&root_node, &context);
    let status = if response.is_ok() { "Ok" } else { "BadRequest" };
    println!("{}", status); // Ok
    let json = serde_json::to_string(&response).unwrap();
    println!("{}", json); // {"data":{"hero":{"name":"R2-D2"}}}
}
