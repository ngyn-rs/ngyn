use ngyn_shared::{HttpMethod, NgynRequest, NgynResponse};

use vercel_runtime::{Body, Error, Request, Response};

use super::{Handler, NgynEngine};

pub struct VercelApplication {
    routes: Vec<(String, HttpMethod, Box<dyn Handler>)>,
}

impl NgynEngine for VercelApplication {
    fn new() -> Self {
        Self { routes: Vec::new() }
    }

    fn route(&mut self, path: &str, method: HttpMethod, handler: Box<impl Handler>) -> &mut Self {
        self.routes.push((path.to_string(), method, handler));
        self
    }
}

impl VercelApplication {
    pub fn get(&mut self, path: &str, handler: impl Handler) -> &mut Self {
        self.route(path, HttpMethod::Get, Box::new(handler))
    }

    pub fn post(&mut self, path: &str, handler: impl Handler) -> &mut Self {
        self.route(path, HttpMethod::Post, Box::new(handler))
    }

    pub async fn handle(self, request: Request) -> Result<Response<Body>, Error> {
        let mut res = NgynResponse::new();
        for (path, method, handler) in self.routes {
            if request.uri().path() == path && request.method().as_str() == method.as_str() {
                let req = NgynRequest::from(request);
                res = handler.handle(req, res);
            }
        }
        Ok(Response::builder()
            .status(res.status_code)
            .body(res.body.into())
            .unwrap())
    }
}
