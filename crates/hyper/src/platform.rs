use http_body_util::BodyExt;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::{service::service_fn, Request};
use hyper_util::rt::TokioIo;
use ngyn_macros::Platform;
use ngyn_shared::response::ResponseBuilder;
use ngyn_shared::{Handler, Method, NgynEngine, NgynResponse};
use std::sync::Arc;
use tokio::net::TcpListener;

/// Represents a Hyper-based application.
#[derive(Default, Platform)]
pub struct HyperApplication {
    routes: Vec<(String, Method, Box<Handler>)>,
    middlewares: Vec<Box<dyn ngyn_shared::NgynMiddleware>>,
}

impl NgynEngine for HyperApplication {
    fn route(&mut self, path: &str, method: Method, handler: Box<Handler>) {
        self.routes.push((path.to_string(), method, handler));
    }

    fn use_middleware(&mut self, middleware: impl ngyn_shared::NgynMiddleware + 'static) {
        self.middlewares.push(Box::new(middleware));
    }
}

impl HyperApplication {
    /// Listens for incoming connections and serves the application.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to listen on.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    pub async fn listen(
        self,
        address: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let server = TcpListener::bind(&address).await?;
        let routes_copy = Arc::new(self.routes);
        let middlewares = Arc::new(self.middlewares);

        let service = service_fn(move |req: Request<Incoming>| {
            let routes = Arc::clone(&routes_copy);
            let middlewares = Arc::clone(&middlewares);
            async move {
                let (parts, mut body) = req.into_parts();
                let body = {
                    let mut buf = Vec::new();
                    if let Some(frame) = body.frame().await {
                        let chunk = frame?.into_data();
                        if let Ok(data) = chunk {
                            buf.extend_from_slice(&data)
                        }
                    }
                    buf
                };
                let req = Request::from_parts(parts, body);
                let res = NgynResponse::init(req, routes, middlewares).await;

                Ok::<_, hyper::Error>(res)
            }
        });
        let http = http1::Builder::new();

        loop {
            tokio::select! {
                Ok((stream, _)) = server.accept() => {
                    let io = TokioIo::new(stream);
                    let conn = http.serve_connection(io, service.clone());

                    tokio::task::spawn(async move {
                        if let Err(e) = conn.await {
                            eprintln!("server error: {}", e);
                        }
                    });
                }
            }
        }
    }
}
