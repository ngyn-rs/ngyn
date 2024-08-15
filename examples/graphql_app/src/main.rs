use std::sync::Arc;

use juniper::{
    tests::fixtures::starwars::schema::{Database, Query},
    EmptyMutation, EmptySubscription, RootNode,
};
use juniper_hyper::{graphiql, graphql, playground};
use ngyn::prelude::*;
use ngyn_hyper::HyperApplication;

#[controller(init = "setup")]
struct GraphQLController {
    db: Arc<Database>,
}

impl GraphQLController {
    fn setup() -> Self {
        Self {
            db: Arc::new(Database::new()),
        }
    }
}

#[routes]
impl GraphQLController {
    #[get("/")]
    fn index(&self, res: &mut NgynResponse) -> &'static str {
        res.headers_mut()
            .append("Content-Type", "text/html".parse().unwrap());
        "You can access the GraphQL playground at <a href='/playground'>/playground</a> or the GraphiQL interface at <a href='/graphiql'>/graphiql</a>."
    }

    #[get("/graphql")]
    #[post("/graphql")]
    async fn routes(&self, req: NgynRequest, res: &mut NgynResponse) {
        let root_node = Arc::new(RootNode::new(
            Query,
            EmptyMutation::<Database>::new(),
            EmptySubscription::<Database>::new(),
        ));
        let req = req.map(|_b| panic!(""));
        let graphql = graphql(root_node, self.db.clone(), req).await;
        *res = graphql.map(|body| body.into());
    }

    #[get("/graphiql")]
    async fn graphiql(&self, res: &mut NgynResponse) {
        let graphiql_res = graphiql("/graphql", None).await;
        *res = graphiql_res.map(|body| body.into());
    }

    #[get("/playground")]
    async fn playground(&self, res: &mut NgynResponse) {
        let playground_res = playground("/graphql", None).await;
        *res = playground_res.map(|body| body.into());
    }
}

#[module(controllers = [GraphQLController])]
struct GraphQLAppModule;

#[tokio::main]
async fn main() {
    let app = NgynFactory::<HyperApplication>::create::<GraphQLAppModule>();
    println!("Listening on http://127.0.0.1:8080");
    let _ = app.listen("127.0.0.1:8080").await;
}
