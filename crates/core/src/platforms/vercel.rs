use http_body_util::Full;
use hyper::{body::Incoming, Request, Response};
use ngyn_macros::platform;
use ngyn_shared::{Bytes, FullResponse, Handler, Method, NgynContext, NgynEngine};
use vercel_runtime::{Body, Error, Response as VercelResponse};

pub struct VercelApplication {
    routes: Vec<(String, Method, Box<Handler>)>,
}

impl NgynEngine for VercelApplication {
    fn new() -> Self {
        Self { routes: Vec::new() }
    }

    fn route(&mut self, path: &str, method: Method, handler: Box<Handler>) -> &mut Self {
        self.routes.push((path.to_string(), method, handler));
        self
    }
}

impl VercelApplication {
    pub async fn handle(self, request: Request<Incoming>) -> Result<VercelResponse<Body>, Error> {
        let mut cx = NgynContext::from_request(request);

        let handler = self
            .routes
            .iter()
            .find(|(path, method, _)| cx.with(path, method).is_some())
            .map(|(_, _, handler)| handler);

        let mut res = Response::new(Full::new(Bytes::default()));

        if let Some(handler) = handler {
            handler(&mut cx, &mut res);
            cx.execute(&mut res).await;
        } else {
            res.set_status(404);
            res.peek("Not Found".to_string());
        }

        Ok(VercelResponse::builder()
            .status(res.status().as_u16())
            .body(String::new().into())
            .unwrap())
    }
}
