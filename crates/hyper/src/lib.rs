use http_body_util::BodyExt;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::{service::service_fn, Request};
use hyper_util::rt::TokioIo;
use hyper_util::server::graceful::GracefulShutdown;
use ngyn_shared::core::engine::{NgynHttpPlatform, PlatformData};
use ngyn_shared::server::NgynResponse;
use std::sync::Arc;
use tokio::net::TcpListener;

#[derive(Default)]
/// Configure an [`HyperApplication`]
pub struct HyperConfig {
    h1_half_close: bool,
    h1_keep_alive: bool,
    h1_title_case_headers: bool,
    h1_preserve_header_case: bool,
    h1_max_headers: Option<usize>,
    max_buf_size: Option<usize>,
    pipeline_flush: bool,
}

/// Represents a Hyper-based application.
#[derive(Default)]
pub struct HyperApplication {
    data: PlatformData,
    config: HyperConfig,
}

impl NgynHttpPlatform for HyperApplication {
    fn data_mut(&mut self) -> &mut PlatformData {
        &mut self.data
    }
}

impl HyperApplication {
    pub fn with_config(config: HyperConfig) -> Self {
        Self {
            data: PlatformData::default(),
            config,
        }
    }
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

        let mut http1 = http1::Builder::new();

        http1
            .half_close(self.config.h1_half_close)
            .keep_alive(self.config.h1_keep_alive)
            .title_case_headers(self.config.h1_title_case_headers)
            .preserve_header_case(self.config.h1_preserve_header_case)
            .pipeline_flush(self.config.pipeline_flush);

        if let Some(buff_size) = self.config.max_buf_size {
            http1.max_buf_size(buff_size);
        }

        if let Some(max_headers) = self.config.h1_max_headers {
            http1.max_headers(max_headers);
        }

        let graceful = GracefulShutdown::new();
        // when this signal completes, start shutdown
        let mut signal = std::pin::pin!(shutdown_signal());

        loop {
            let data = data.clone();
            tokio::select! {
                Ok((stream, _)) = server.accept() => {
                    let io = TokioIo::new(stream);
                    let conn = http1.serve_connection(io, service_fn(move |req| hyper_service(data.clone(), req)));
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

async fn hyper_service(
    data: Arc<PlatformData>,
    req: Request<Incoming>,
) -> Result<NgynResponse, hyper::Error> {
    let (parts, mut body) = req.into_parts();
    let body = {
        let mut buf = Vec::new();
        // TODO: change this approach. It's not efficient.
        while let Some(frame) = body.frame().await {
            if let Ok(bytes) = frame?.into_data() {
                buf.extend_from_slice(&bytes);
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

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for signal");
}
