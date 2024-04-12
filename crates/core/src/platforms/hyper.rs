use std::sync::Arc;

use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::{service::service_fn, Request, Response};
use ngyn_shared::{Handler, HttpMethod, NgynEngine, NgynContext};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

pub struct HyperApplication {
    routes: Vec<(String, HttpMethod, Box<dyn Handler>)>,
}

impl NgynEngine for HyperApplication {
    fn new() -> Self {
        HyperApplication { routes: Vec::new() }
    }

    fn route(&mut self, path: &str, method: HttpMethod, handler: Box<impl Handler>) -> &mut Self {
        self.routes.push((path.to_string(), method, handler));
        self
    }
}

impl HyperApplication {
    pub async fn listen(self, address: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let server = TcpListener::bind(&address).await?;
        let routes_copy = Arc::new(self.routes);

        let service = service_fn(move |req: Request<Incoming>| {
            let routes_copy = Arc::clone(&routes_copy);
            async move {
                let req = Request::from(req);
                let mut res = Response::new(Full::new(Bytes::default()));

                let mut cx = NgynContext::from_request(req);

                for (path, _, handler) in routes_copy.iter() {
                    if let Some(cx) = cx.with(&path) {
                        let mut cx = cx;
                        handler.handle(&mut cx, &mut res);
                        break;
                    }
                };

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
