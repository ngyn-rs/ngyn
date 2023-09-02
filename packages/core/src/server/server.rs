use rustle_shared::{HttpMethod, RustleRequest, RustleResponse};
use tide::Server;

/// `RustleServer` is a struct that represents a server instance in the Rustle framework.
/// It contains a `Server` from the Tide framework and an optional `Route`.
pub struct RustleServer {
    server: Server<()>,
}

impl RustleServer {
    /// Creates a new instance of `RustleServer` with a new `Server`
    pub fn new() -> Self {
        Self {
            server: Server::new(),
        }
    }

    /// Adds a new route to the `RustleServer`.
    /// This function is chainable.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that represents the path of the route.
    /// * `method` - An `HttpMethod` that represents the HTTP method of the route.
    /// * `handler` - A closure that takes a `RustleRequest` and a `RustleResponse` and returns a `RustleResponse`.
    ///
    /// # Example
    ///
    /// ```
    /// let mut server = RustleServer::new();
    /// server.route("/", HttpMethod::Get, |req, res| {
    ///    res.status(200)
    /// });
    /// ```
    pub fn route(
        &mut self,
        path: &str,
        method: HttpMethod,
        handler: &Box<dyn Fn(RustleRequest, RustleResponse) -> RustleResponse + Send + Sync>,
    ) -> &mut Self {
        let handler = std::sync::Arc::new(handler);
        let req_handler = move |req: tide::Request<()>| async move {
            let rustle_request = RustleRequest::new(req);
            let rustle_response = RustleResponse::new();
            rustle_response.body("Hello World").build()
        };
        match method {
            HttpMethod::Get => self.server.at(path).get(req_handler),
            HttpMethod::Post => self.server.at(path).post(req_handler),
            HttpMethod::Put => self.server.at(path).put(req_handler),
            HttpMethod::Delete => self.server.at(path).delete(req_handler),
            _ => panic!("Unsupported HTTP method"),
        };
        self
    }

    /// Starts listening for incoming connections on the specified address.
    /// This function is asynchronous and returns a `tide::Result`.
    pub async fn listen(self, address: &str) -> tide::Result<()> {
        self.server.listen(address).await.map_err(tide::Error::from)
    }
}
