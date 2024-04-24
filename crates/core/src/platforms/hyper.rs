use http_body_util::{BodyExt, Full};
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::{service::service_fn, Request, Response};
use hyper_util::rt::TokioIo;
use ngyn_macros::Platform;
use ngyn_shared::{FullResponse, Handler, Method, NgynContext, NgynEngine};
use std::sync::Arc;
use tokio::net::TcpListener;

#[derive(Default, Platform)]
pub struct HyperApplication {
    routes: Vec<(String, Method, Option<Box<Handler>>)>,
}

impl NgynEngine for HyperApplication {
    fn route(&mut self, path: &str, method: Method, handler: Box<Handler>) -> &mut Self {
        self.routes.push((path.to_string(), method, Some(handler)));
        self
    }
}

impl HyperApplication {
    pub async fn listen(
        self,
        address: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let server = TcpListener::bind(&address).await?;
        let routes_copy = Arc::new(self.routes);

        let service = service_fn(move |req: Request<Incoming>| {
            let routes = Arc::clone(&routes_copy);
            async move {
                let mut cx = NgynContext::from_request(req.map(|b| {
                    let mut new_body = vec![];
                    b.map_frame(|mut f| {
                        if let Some(d) = f.data_mut() {
                            new_body.append(&mut d.to_vec());
                        }
                        f
                    });
                    new_body
                }));
                let mut res = Response::new(Full::new(Bytes::default()));

                let handler = routes
                    .iter()
                    .filter_map(|(path, method, handler)| {
                        if cx.with(path, method).is_some() {
                            Some(handler)
                        } else {
                            None
                        }
                    })
                    .next();

                if let Some(Some(handler)) = handler {
                    handler(&mut cx, &mut res);
                    cx.execute(&mut res).await;
                } else {
                    res.set_status(404);
                    res.peek("Not Found".to_string());
                }

                Ok::<_, hyper::Error>(res)
            }
        });

        loop {
            let (stream, _) = server.accept().await?;
            let io = TokioIo::new(stream);

            let http = http1::Builder::new();
            let conn = http.serve_connection(io, service.clone());

            tokio::task::spawn(async move {
                if let Err(e) = conn.await {
                    eprintln!("server error: {}", e);
                }
            });
        }
    }
}
