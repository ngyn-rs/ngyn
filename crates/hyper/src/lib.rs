use http_body_util::BodyExt;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::{service::service_fn, Request};
use hyper_util::rt::TokioIo;
use ngyn_shared::core::{NgynPlatform, PlatformData};
use std::sync::Arc;
use tokio::net::TcpListener;

/// Represents a Hyper-based application.
#[derive(Default)]
pub struct HyperApplication {
    data: PlatformData,
}

impl NgynPlatform for HyperApplication {
    fn data_mut(&mut self) -> &mut PlatformData {
        &mut self.data
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
    pub async fn listen<A: tokio::net::ToSocketAddrs>(
        self,
        address: A,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let server = TcpListener::bind(address).await?;

        let data = Arc::new(self.data);

        let service = service_fn(move |req: Request<Incoming>| {
            let data = data.clone();
            async move {
                let (parts, mut body) = req.into_parts();
                let body = {
                    let mut buf = Vec::new();
                    if let Some(frame) = body.frame().await {
                        let chunk = frame?.into_data();
                        if let Ok(bytes) = chunk {
                            buf.extend_from_slice(&bytes)
                        }
                    }
                    buf
                };
                let req = Request::from_parts(parts, body);
                let res = data.respond(req).await;

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
