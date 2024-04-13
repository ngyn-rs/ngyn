use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::{service::service_fn, Request, Response};
use hyper_util::rt::TokioIo;
use ngyn_shared::{FullResponse, Handler, HttpMethod, NgynContext, NgynEngine};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

pub struct HyperApplication {
    routes: Arc<Mutex<Vec<(String, HttpMethod, Option<Box<Handler>>)>>>,
}

impl NgynEngine for HyperApplication {
    fn new() -> Self {
        HyperApplication {
            routes: Arc::new(Mutex::new(vec![])),
        }
    }

    fn route(&mut self, path: &str, method: HttpMethod, handler: Box<Handler>) -> &mut Self {
        self.routes
            .lock()
            .unwrap()
            .push((path.to_string(), method, Some(handler)));
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
            let routes_copy = Arc::clone(&routes_copy);
            async move {
                let mut cx = NgynContext::from_request(req);
                let mut res = Response::new(Full::new(Bytes::default()));

                let routes = routes_copy.lock().unwrap();
                let handler = routes
                    .iter()
                    .find(|(path, method, _)| cx.with(path, method).is_some())
                    .map(|(_, _, handler)| handler);

                if let Some(Some(handler)) = handler {
                    handler(&mut cx, &mut res);
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
