use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::{service::service_fn, Request, Response};
use ngyn_shared::{Handler, HttpMethod, NgynEngine, ToParts};
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

        let service = service_fn(|req: Request<Incoming>| async move {
            let mut res = Response::new(Full::new(Bytes::default()));

            for (path, method, handler) in self.routes.iter() {
                let (is_match, parts) = req.uri().parts(path);
                if is_match {
                    handler.handle(req, res);
                }
            }

            Ok::<_, hyper::Error>(res)
        });

        loop {
            let (stream, _) = server.accept().await?;
            let io = TokioIo::new(stream);

            tokio::task::spawn(async move {
                let http = http1::Builder::new();
                let conn = http.serve_connection(io, service);

                if let Err(e) = conn.await {
                    eprintln!("server error: {}", e);
                }
            });
        }
    }
}
