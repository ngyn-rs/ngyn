use std::sync::Arc;

use http_body_util::Full;
use juniper::{
    tests::fixtures::starwars::schema::{Database, Query},
    EmptyMutation, EmptySubscription, RootNode,
};
use juniper_hyper::{graphiql, graphql, playground};
use ngyn::prelude::*;
use ngyn_hyper::HyperApplication;

#[controller]
struct GraphQLController {
    db: Database,
}

#[routes]
impl GraphQLController {
    #[get("/graphql")]
    #[post("/graphql")]
    async fn routes(&self, req: NgynRequest, res: &mut NgynResponse) {
        let root_node = Arc::new(RootNode::new(
            Query,
            EmptyMutation::<Database>::new(),
            EmptySubscription::<Database>::new(),
        ));
        let req =
            req.map(|_b| panic!("There's currently no way to transform this body correctly."));
        let graphql = graphql(root_node, Arc::new(self.db.clone()), req).await;
        *res = graphql.map(|body| Full::new(Bytes::from(body)));
    }

    #[get("/graphiql")]
    async fn graphiql(&self, res: &mut NgynResponse) {
        let graphiql_res = graphiql("/graphql", None).await;
        *res = graphiql_res.map(|body| Full::new(Bytes::from(body)));
    }

    #[get("/playground")]
    async fn playground(&self, res: &mut NgynResponse) {
        let playground_res = playground("/graphql", None).await;
        *res = playground_res.map(|body| Full::new(Bytes::from(body)));
    }
}

#[module(controllers = [GraphQLController])]
struct GraphQLAppModule;

#[tokio::main]
async fn main() {
    let app = NgynFactory::<HyperApplication>::create::<GraphQLAppModule>();
    let _ = app.listen("127.0.0.1:8080").await;
}
