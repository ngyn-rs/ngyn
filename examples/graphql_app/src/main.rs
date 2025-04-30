use juniper::{
    tests::fixtures::starwars::schema::{Database, Query},
    EmptyMutation, EmptySubscription, RootNode,
};
use juniper_hyper::{graphiql, graphql, playground};
use ngyn::prelude::*;
use ngyn_hyper::HyperApplication;
use std::sync::Arc;

#[handler]
fn home() -> &'static str {
    "<html><body>You can access the GraphQL playground at <a href='/playground'>/playground</a> or the GraphiQL interface at <a href='/graphiql'>/graphiql</a>.</body></html>"
}

#[handler]
async fn handle_graphql(req: NgynRequest) -> String {
    let db = Arc::new(Database::new());
    let root_node = Arc::new(RootNode::new(
        Query,
        EmptyMutation::<Database>::new(),
        EmptySubscription::<Database>::new(),
    ));
    let graphql_res = graphql(root_node, db, req.map(|_b| panic!(""))).await;
    graphql_res.body().as_str().to_owned()
}

#[handler]
async fn handle_graphiql() -> String {
    let graphiql_res = graphiql("/graphql", None).await;
    graphiql_res.body().as_str().to_owned()
}

#[handler]
async fn handle_playground() -> String {
    let playground_res = playground("/graphql", None).await;
    playground_res.body().as_str().to_owned()
}

#[tokio::main]
async fn main() {
    let mut app = HyperApplication::default();

    app.get("/", home);

    app.get("/playground", async_wrap(handle_playground));

    app.get("/graphiql", async_wrap(handle_graphiql));

    app.any("/graphql", async_wrap(handle_graphql));

    println!("Listening on http://127.0.0.1:8080");

    let _ = app.listen("127.0.0.1:8080").await;
}
