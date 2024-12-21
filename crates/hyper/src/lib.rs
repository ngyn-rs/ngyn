use http_body_util::BodyExt;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::{service::service_fn, Request};
use hyper_util::rt::TokioIo;
use ngyn_shared::core::engine::{NgynHttpPlatform, PlatformData};
use ngyn_shared::server::NgynResponse;
use std::future::Future;
use std::sync::Arc;
use tokio::net::TcpListener;

/// Represents a Hyper-based application.
#[derive(Default)]
pub struct HyperApplication {
    data: PlatformData,
}

impl NgynHttpPlatform for HyperApplication {
    fn data_mut(&mut self) -> &mut PlatformData {
        &mut self.data
    }
}

impl HyperApplication {
    /// Listens for incoming connections and serves the application.
    ///
    /// ### Arguments
    ///
    /// * `address` - The address to listen on.
    ///
    /// ### Returns
    ///
    /// A `Result` indicating success or failure.
    pub async fn listen<A: tokio::net::ToSocketAddrs>(
        self,
        address: A,
    ) -> Result<(), std::io::Error> {
        let server = TcpListener::bind(address).await?;
        let data = Arc::new(self.data);

        let http = http1::Builder::new();
        let graceful = hyper_util::server::graceful::GracefulShutdown::new();
        // when this signal completes, start shutdown
        let mut signal = std::pin::pin!(shutdown_signal());

        loop {
            let data = data.clone();
            tokio::select! {
                Ok((stream, _)) = server.accept() => {
                    let io = TokioIo::new(stream);
                    let conn = http.serve_connection(io, service_fn(move |req| hyper_service(data.clone(), req)));
                    let handle = graceful.watch(conn);

                    tokio::task::spawn(async move {
                        if let Err(e) = handle.await {
                            eprintln!("server connection error: {}", e);
                        }
                    });
                }
                _ = &mut signal => {
                    eprintln!("graceful shutdown signal received");
                    // stop the accept loop
                    break;
                }
                else => continue, // continue waiting for the next signal or connection
            }
        }

        tokio::select! {
            _ = graceful.shutdown() => {
                eprintln!("all connections gracefully closed");
            },
            _ = tokio::time::sleep(std::time::Duration::from_secs(10)) => {
                eprintln!("timed out wait for all connections to close");
            }
        }

        Ok(())
    }
}

fn hyper_service(
    data: Arc<PlatformData>,
    req: Request<Incoming>,
) -> impl Future<Output = Result<NgynResponse, hyper::Error>> {
    async move {
        let (parts, mut body) = req.into_parts();
        let body = {
            let mut buf = Vec::new();
            // TODO: change this approach. It's not efficient.
            loop {
                if let Some(frame) = body.frame().await {
                    let chunk = frame?.into_data();
                    if let Ok(bytes) = chunk {
                        buf.extend_from_slice(&bytes);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            buf
        };
        let req = Request::from_parts(parts, body);
        let res = data.respond(req).await;

        Ok::<_, hyper::Error>(res)
    }
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for signal");
}
