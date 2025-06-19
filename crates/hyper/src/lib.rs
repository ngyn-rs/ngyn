//! Hyper integration for Ngyn web framework.
//!
//! This module provides a Hyper-based HTTP server implementation for Ngyn applications.

use std::{error::Error, sync::Arc, time::Duration};

use http_body_util::BodyExt;
use hyper::{body::Incoming, server::conn::http1, service::service_fn, Request};
use hyper_util::{rt::TokioIo, server::graceful::GracefulShutdown};
use ngyn_shared::{
    core::{NgynHttpPlatform, PlatformData},
    server::NgynResponse,
};
use tokio::{net::TcpListener, sync::mpsc};

const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(10);
const ERROR_CHANNEL_SIZE: usize = 100;

/// Configuration options for the [`HyperApplication`].
///
/// This struct provides various HTTP/1.1 specific configuration options
/// for customizing the behavior of the Hyper server.
#[derive(Default, Debug, Clone)]
pub struct HyperConfig {
    h1_half_close: bool,
    h1_keep_alive: bool,
    h1_title_case_headers: bool,
    h1_preserve_header_case: bool,
    h1_max_headers: Option<usize>,
    max_buf_size: Option<usize>,
    pipeline_flush: bool,
}

/// A Hyper-based HTTP server implementation for Ngyn applications.
///
/// This struct implements the [`NgynHttpPlatform`] trait and provides
/// a complete HTTP server implementation using Hyper.
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
    /// Creates a new `HyperApplication` with the specified configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use ngyn_hyper::{HyperApplication, HyperConfig};
    ///
    /// let config = HyperConfig::default();
    /// let app = HyperApplication::with_config(config);
    /// ```
    #[must_use]
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
    /// * `address` - The address to listen on. This can be any type that implements
    ///   the `tokio::net::ToSocketAddrs` trait, such as `&str`, `SocketAddr`, or `(String, u16)`.
    ///
    /// ### Examples
    ///
    /// ```no_run
    /// use ngyn_hyper::HyperApplication;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let app = HyperApplication::default();
    ///     
    ///     // Listen on localhost:3000
    ///     app.listen("127.0.0.1:3000").await;
    /// }
    /// ```
    ///
    /// You can also use a socket address:
    ///
    /// ```no_run
    /// use std::net::SocketAddr;
    /// use ngyn_hyper::HyperApplication;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let addr: SocketAddr = "[::1]:8080".parse().unwrap();
    ///     let app = HyperApplication::default();
    ///     
    ///     app.listen(addr).await;
    /// }
    /// ```
    pub async fn listen<A: tokio::net::ToSocketAddrs>(self, address: A) {
        let http1 = self.build_http1_config();
        let server = match TcpListener::bind(address).await {
            Ok(server) => server,
            Err(err) => {
                handle_error(&self.data, err);
                return;
            }
        };
        let graceful = GracefulShutdown::new();
        let signal = std::pin::pin!(shutdown_signal());

        self.run_server(server, http1, graceful, signal).await
    }

    fn build_http1_config(&self) -> http1::Builder {
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

        http1
    }

    async fn run_server(
        self,
        server: TcpListener,
        http1: http1::Builder,
        graceful: GracefulShutdown,
        mut signal: std::pin::Pin<&mut impl std::future::Future<Output = ()>>,
    ) {
        let data = Arc::new(self.data);
        let (error_sender, mut error_receiver) = mpsc::channel::<hyper::Error>(ERROR_CHANNEL_SIZE);

        loop {
            let data = data.clone();
            tokio::select! {
                Ok((mut stream, _)) = server.accept() => {
                    if !is_valid_http_version(&mut stream).await {
                        continue;
                    }

                    let io = TokioIo::new(stream);
                    let conn = http1.serve_connection(
                        io,
                        service_fn(move |req| hyper_service(data.clone(), req)),
                    );
                    let handle = graceful.watch(conn);
                    let error_sender = error_sender.clone();

                    tokio::task::spawn(async move {
                        if let Err(e) = handle.await {
                            let _ = error_sender.try_send(e);
                        }
                    });
                }
                Some(err) = error_receiver.recv() => {
                    handle_error(&data, err);
                }
                _ = &mut signal => break,
                else => continue,
            }
        }

        if let Err(err) = handle_shutdown(graceful, error_receiver).await {
            handle_error(&data, err);
        }
    }
}

async fn is_valid_http_version(stream: &mut tokio::net::TcpStream) -> bool {
    // Most HTTP servers limit request line to 8KB, but we'll use 1KB as a reasonable compromise
    let mut buffer = [0u8; 1024];
    match stream.peek(&mut buffer).await {
        Ok(n) if n >= 8 => {
            if let Ok(Some(request_line)) = std::str::from_utf8(&buffer[..n])
                .map(|data| data.find("\r\n").map(|line_end| &data[..line_end]))
            {
                return request_line.ends_with("HTTP/1.1") || request_line.ends_with("HTTP/1.0");
            }
            false
        }
        _ => false, // If we can't peek or get enough data, assume invalid
    }
}

fn handle_error(data: &PlatformData, err: impl Error + Sync + Send + 'static) {
    if let Some(handler) = data.error_handler() {
        handler(ngyn_shared::core::NgynError::Other(Box::new(err)));
    } else {
        eprintln!("Server error occurred: {}", err);
    }
}

async fn handle_shutdown(
    graceful: GracefulShutdown,
    mut error_receiver: mpsc::Receiver<hyper::Error>,
) -> Result<(), std::io::Error> {
    tokio::select! {
        _ = graceful.shutdown() => Ok(()),
        Some(err) = error_receiver.recv() => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                err
            )),
        _ = tokio::time::sleep(SHUTDOWN_TIMEOUT) => {
            Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "graceful shutdown timed out",
            ))
        }
    }
}

type HyperResult<T> = Result<T, hyper::Error>;

/// Handles incoming HTTP requests and converts them into Ngyn responses.
///
/// This function serves as the core request handler, processing incoming
/// HTTP requests and producing appropriate responses using the application's
/// request handling logic.
///
/// This function performs the following steps:
/// 1. Splits the incoming request into parts and body
/// 2. Collects the entire body into a buffer
/// 3. Reconstructs the request with the collected body
/// 4. Processes the request through the Ngyn platform
///
/// # Note
///
/// The current body handling approach buffers the entire request body in memory.
/// This is not ideal for large requests and should be improved in future versions
/// to use streaming where possible.
async fn hyper_service(
    data: Arc<PlatformData>,
    req: Request<Incoming>,
) -> HyperResult<NgynResponse> {
    let (parts, body) = req.into_parts();
    let body = collect_body(body).await?;
    let req = Request::from_parts(parts, body);

    Ok(data.respond(req).await)
}

/// Collects the entire body of an incoming request into a vector.
///
/// # Note
///
/// This is a temporary solution. Future implementations should consider:
/// - Streaming support for large bodies
/// - Memory limits for request bodies
/// - Proper error handling for malformed bodies
async fn collect_body(mut body: Incoming) -> HyperResult<Vec<u8>> {
    let mut buf = Vec::new();

    while let Some(frame) = body.frame().await {
        if let Ok(bytes) = frame?.into_data() {
            buf.extend_from_slice(&bytes);
        } else {
            break;
        }
    }
    Ok(buf)
}

/// Waits for a shutdown signal (Ctrl+C) to be received.
///
/// # Panics
///
/// Panics if the signal handler cannot be installed.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C handler")
}
