use std::str::FromStr;

use http_body_util::{BodyExt, Full};
use hyper::{header::HeaderName, Response, Uri};
use ngyn_macros::platform;
use ngyn_shared::{Bytes, FullResponse, Handler, Method, NgynContext, NgynEngine};
use vercel_runtime::{Body, Error, Request, Response as VercelResponse};

#[platform]
pub struct VercelApplication {
    routes: Vec<(String, Method, Box<Handler>)>,
}

impl Default for VercelApplication {
    fn default() -> Self {
        Self { routes: vec![] }
    }
}

impl NgynEngine for VercelApplication {
    fn route(&mut self, path: &str, method: Method, handler: Box<Handler>) -> &mut Self {
        self.routes.push((path.to_string(), method, handler));
        self
    }
}

impl VercelApplication {
    pub async fn handle(self, request: Request) -> Result<VercelResponse<Body>, Error> {
        let request = request.map(|b| b.to_vec());
        let (parts, body) = request.into_parts();
        let request = {
            let mut hyper_request = hyper::Request::new(body);

            let method = parts.method.to_string();
            *hyper_request.method_mut() = Method::from_bytes(method.as_bytes()).unwrap();

            let uri = parts.uri.clone().to_string();
            *hyper_request.uri_mut() = Uri::from_str(&uri).unwrap();

            let headers = hyper_request.headers_mut();
            let raw_headers = parts.headers.clone();
            raw_headers.into_iter().for_each(|(key, value)| {
                if let Some(key) = key {
                    headers.insert(
                        HeaderName::from_str(key.as_str()).unwrap(),
                        value.clone().to_str().unwrap().parse().unwrap(),
                    );
                }
            });

            hyper_request
        };
        let mut cx = NgynContext::from_request(request);

        let handler = self
            .routes
            .iter()
            .filter_map(|(path, method, handler)| {
                if cx.with(path, method).is_some() {
                    Some(handler)
                } else {
                    None
                }
            })
            .next();

        let mut res = Response::new(Full::new(Bytes::default()));

        if let Some(handler) = handler {
            handler(&mut cx, &mut res);
            cx.execute(&mut res).await;
        } else {
            res.set_status(404);
            res.peek("Not Found".to_string());
        }

        let body = {
            let mut body = Vec::new();

            res.body_mut().map_frame(|f| {
                body.extend_from_slice(&f.data_ref().unwrap().to_vec());
                f
            });

            body
        };

        Ok(VercelResponse::builder()
            .status(res.status().as_u16())
            .body(body.into())
            .unwrap())
    }
}
