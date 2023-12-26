use ngyn_macros::platform;
use ngyn_shared::{Handler, HttpMethod, NgynBody, NgynEngine, NgynRequest};
use vercel_runtime::{Body, Error, Request, Response};

#[platform]
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
    pub async fn handle(self, request: Request) -> Result<Response<Body>, Error> {
        let mut is_found = false;
        let mut res = ngyn_shared::NgynResponse::from_status(200);
        let (parts, body) = request.into_parts();

        for (path, method, handler) in self.routes {
            let uri = parts.uri.clone();
            if uri.path() == path && parts.method.as_str() == method.as_str() {
                let headers = {
                    let mut entries = std::collections::HashMap::new();
                    for (name, value) in parts.headers.clone().into_iter() {
                        if let Some(name) = name {
                            let value = value.to_str().unwrap();
                            entries.insert(name.to_string(), value.to_string());
                        }
                    }
                    entries
                };
                let mut req = NgynRequest::from((
                    parts.method.to_string(),
                    uri.to_string(),
                    headers,
                    body.to_vec(),
                ));
                handler.handle(&mut req, &mut res);

                // Wait for the response to be ready
                res = res.await;
                is_found = true;
                break; // Exit the loop once a route is found
            }
        }

        // if the response is a 404 and has no body, send "Not Found"
        if !is_found {
            res.set_status(404);
            res.send("Not Found");
        }

        let mut body_str = String::new();

        match res.body_raw() {
            NgynBody::String(body) => {
                body_str = body;
            }
            NgynBody::Bool(body) => {
                body_str = body.to_string();
            }
            NgynBody::Number(body) => {
                body_str = body.to_string();
            }
            NgynBody::Map(body) => {
                for (key, value) in body {
                    let value_str: String = value.into();
                    body_str.push_str(&format!("{}: {}\n", key, value_str));
                }
            }
            NgynBody::None => {}
        }

        Ok(Response::builder()
            .status(res.status())
            .body(body_str.into())
            .unwrap())
    }
}
