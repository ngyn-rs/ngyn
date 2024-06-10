use http_body_util::BodyExt;
use ngyn_macros::Platform;
use ngyn_shared::{response::ResponseBuilder, Handler, Method, NgynEngine, NgynResponse};
use std::sync::Arc;
use vercel_runtime::{Body, Error, Request, Response as VercelResponse};

#[derive(Default, Platform)]
pub struct VercelApplication {
    routes: Vec<(String, Method, Box<Handler>)>,
    middlewares: Vec<Box<dyn ngyn_shared::NgynMiddleware>>,
}

impl NgynEngine for VercelApplication {
    fn route(&mut self, path: &str, method: Method, handler: Box<Handler>) {
        self.routes.push((path.to_string(), method, handler));
    }

    fn use_middleware(&mut self, middleware: impl ngyn_shared::NgynMiddleware + 'static) {
        self.middlewares.push(Box::new(middleware));
    }
}

impl VercelApplication {
    pub async fn handle(self, request: Request) -> Result<VercelResponse<Body>, Error> {
        let request = request.map(|b| b.to_vec());

        let response =
            NgynResponse::init(request, Arc::new(self.routes), Arc::new(self.middlewares)).await;

        let (parts, mut body) = response.into_parts();
        let body = {
            let mut buf = Vec::new();
            let f = body.frame().await.unwrap().unwrap();
            let d = f.data_ref().unwrap();
            buf.extend_from_slice(d.to_vec().as_slice());
            Body::from(buf)
        };

        let response = VercelResponse::from_parts(parts, body);

        Ok(response)
    }
}
