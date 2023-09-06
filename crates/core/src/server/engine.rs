use ngyn_shared::{NgynRequest, NgynResponse};
use tide::{utils::async_trait, Middleware, Next, Request};

pub struct NgynEngine;

#[async_trait]
impl Middleware<()> for NgynEngine {
    async fn handle(&self, req: Request<()>, _next: Next<'_, ()>) -> tide::Result {
        let mut _ngyn_response = NgynResponse::new();
        let _ngyn_request = NgynRequest::new(req);

        // TODO: Implement routing logic here

        _ngyn_response.build()
    }
}
