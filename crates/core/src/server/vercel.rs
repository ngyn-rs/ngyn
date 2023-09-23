use ngyn_shared::HttpMethod;

#[cfg(feature = "vercel")]
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

#[cfg(feature = "vercel")]
impl VercelApplication {
    pub fn get(&mut self, path: &str, handler: impl Handler) -> &mut Self {
        self.route(path, HttpMethod::Get, Box::new(handler))
    }

    pub fn post(&mut self, path: &str, handler: impl Handler) -> &mut Self {
        self.route(path, HttpMethod::Post, Box::new(handler))
    }

    pub async fn handle(self, request: Request) -> Result<Response<Body>, Error> {
        let mut res = ngyn_shared::NgynResponse::new();
        let (parts, body) = request.into_parts();
        let mut found_route = false;
        for (path, method, handler) in self.routes {
            let uri = parts.uri.clone();
            if uri.path() == path && parts.method.as_str() == method.as_str() {
                let headers = {
                    let mut entries = std::collections::HashMap::new();
                    for (name, value) in parts.headers.clone().into_iter() {
                        match name {
                            Some(name) => {
                                let value = value.to_str().unwrap();
                                entries.insert(name.to_string(), value.to_string());
                            }
                            None => todo!(), // TODO: Figure out what to do if the header key is None
                        }
                    }
                    entries
                };
                let req = ngyn_shared::NgynRequest::from((
                    parts.method.to_string(),
                    uri.to_string(),
                    headers,
                    body.to_vec(),
                ));
                res = handler.handle(req, res).await;
                found_route = true;
            }
        }

        if !found_route {
            // Handle case where route is not found
            // Return a 404 Not Found response
            res.status_code = 404;
            res.raw_body = "Route not found".to_string();
        }

        Ok(Response::builder()
            .status(res.status_code)
            .body(res.raw_body.into())
            .unwrap())
    }
}
