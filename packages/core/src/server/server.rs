use rustle_shared::{HttpMethod, RustleModule};
use tide::{http::Method, Route, Server};

use super::{request::RustleRequest, response::RustleResponse};

/// `RustleServer` is a struct that represents a server instance in the Rustle framework.
/// It contains a `Server` from the Tide framework and an optional `Route`.
pub struct RustleServer {
    server: Server,
    route: Option<Route>,
}

impl RustleServer {
    /// Creates a new instance of `RustleServer` with a new `Server` and no `Route`.
    pub fn new() -> Self {
        Self {
            server: Server::new(),
            route: None,
        }
    }

    /// Sets the route of the `RustleServer` to the specified path.
    /// Returns a mutable reference to the `RustleServer`.
    pub fn at(&mut self, path: &str) -> &mut Self {
        self.route = Some(self.server.at(path));
        self
    }

    /// Sets the HTTP method and handler for the current route of the `RustleServer`.
    /// The handler is a function that takes a `RustleRequest` and a `RustleResponse` and returns nothing.
    /// If the route is not set, this function will panic.
    /// Returns a mutable reference to the `RustleServer`.
    pub fn method(
        &mut self,
        method: HttpMethod,
        handler: impl Fn(RustleRequest, RustleResponse) + Send + Sync + 'static,
    ) -> &mut Self {
        if let Some(route) = &mut self.route {
            match method {
                HttpMethod::Get => route.get(handler),
                HttpMethod::Post => route.post(handler),
                HttpMethod::Put => route.put(handler),
                HttpMethod::Delete => route.delete(handler),
                _ => panic!("Unsupported HTTP method"),
            };
        } else {
            panic!("Route is not set. Please set the route before setting the method.");
        }
        self
    }

    /// Starts listening for incoming connections on the specified address.
    /// This function is asynchronous and returns a `tide::Result`.
    pub async fn listen(&mut self, address: &str) -> tide::Result<()> {
        self.server.listen(address).await
    }
}
