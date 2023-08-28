use rustle_shared::{RustleRequest, RustleResponse};
use tide::{utils::async_trait, Middleware, Next, Request};

pub struct RustleEngine;

#[async_trait]
impl Middleware<()> for RustleEngine {
    async fn handle(&self, req: Request<()>, _next: Next<'_, ()>) -> tide::Result {
        let mut _rustle_response = RustleResponse::new();
        let _rustle_request = RustleRequest::new(req);

        // TODO: Implement routing logic here

        _rustle_response.build()
    }
}
