use http_body_util::BodyExt;
use hyper::{header::HeaderName, Uri};
use ngyn_macros::Platform;
use ngyn_shared::{response::ResponseBuilder, Handler, Method, NgynEngine, NgynResponse};
use std::{str::FromStr, sync::Arc};
use vercel_runtime::{Body, Error, Request, Response as VercelResponse};

#[derive(Default, Platform)]
pub struct VercelApplication {
    routes: Vec<(String, Method, Box<Handler>)>,
}

impl NgynEngine for VercelApplication {
    fn route(&mut self, path: &str, method: Method, handler: Box<Handler>) {
        self.routes.push((path.to_string(), method, handler));
    }
}

impl VercelApplication {
    pub async fn handle(self, request: Request) -> Result<VercelResponse<Body>, Error> {
        let request = request.map(|b| b.to_vec());
        let (parts, body) = request.into_parts();

        // TODO: Once vercel_runtime supports http v1, we can remove this
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
        let res = NgynResponse::init(request, Arc::new(self.routes)).await;

        // TODO: Once vercel_runtime supports http v1, we can remove this
        let (parts, body) = {
            let (parts, mut r_body) = res.into_parts();
            (
                parts,
                r_body
                    .frame()
                    .await
                    .unwrap()
                    .unwrap()
                    .into_data()
                    .unwrap()
                    .to_vec(),
            )
        };

        Ok(VercelResponse::builder()
            .status(parts.status.as_u16())
            .body(body.into())
            .unwrap())
    }
}
