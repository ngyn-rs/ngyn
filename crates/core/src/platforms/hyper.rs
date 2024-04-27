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
}

impl NgynEngine for HyperApplication {
    fn route(&mut self, path: &str, method: Method, handler: Box<Handler>) {
        self.routes.push((path.to_string(), method, handler));
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

        let service = service_fn(move |req: Request<Incoming>| {
            let routes = Arc::clone(&routes_copy);
            async move {
                let req = req.map(|b| {
                    let mut new_body = vec![];
                    b.map_frame(|f| {
                        if let Some(d) = f.data_ref() {
                            new_body.append(&mut d.to_vec());
                        }
                        f
                    });
                    new_body
                });
                let res = NgynResponse::init(req, routes).await;

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
