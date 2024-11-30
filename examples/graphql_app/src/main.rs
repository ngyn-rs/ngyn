use hyper::Request;
use juniper::{
    tests::fixtures::starwars::schema::{Database, Query},
    EmptyMutation, EmptySubscription, RootNode,
};
use juniper_hyper::{graphiql, graphql, playground};
use ngyn::prelude::*;
use ngyn_hyper::HyperApplication;
use std::sync::Arc;

#[handler]
async fn handle_graphql() {
    let db = Arc::new(Database::new());
    let root_node = Arc::new(RootNode::new(
        Query,
        EmptyMutation::<Database>::new(),
        EmptySubscription::<Database>::new(),
    ));
    let b = response;
    let graphql_res = graphql(
        root_node.clone(),
        db.clone(),
        Request::default().map(|_b: String| panic!("")),
    )
    .await;
    graphql_res.body().as_str().to_owned()
}

#[tokio::main]
async fn main() {
    let mut app = HyperApplication::default();

    app.get("/", handler(|_c| "<html><body>You can access the GraphQL playground at <a href='/playground'>/playground</a> or the GraphiQL interface at <a href='/graphiql'>/graphiql</a>.</body></html>"));

    app.get(
        "/playground",
        async_handler(|_c| async {
            let playground_res = playground("/graphql", None).await;
            playground_res.body().as_str().to_owned()
        }),
    );

    app.get(
        "/graphiql",
        async_handler(|_c| async {
            let graphiql_res = graphiql("/graphql", None).await;
            graphiql_res.body().as_str().to_owned()
        }),
    );

    app.any("/graphql", RouteHandler::Async(Box::new(handle_graphql)));

    println!("Listening on http://127.0.0.1:8080");
    let _ = app.listen("127.0.0.1:8080").await;
}
