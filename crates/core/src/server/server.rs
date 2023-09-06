use ngyn_shared::{HttpMethod, NgynRequest, NgynResponse};
use std::sync::Arc;
use tide::{Result, Server};

/// `NgynServer` is a struct that represents a server instance in the Ngyn framework.
pub struct NgynServer {
    server: Server<()>,
}

impl NgynServer {
    /// Creates a new instance of `NgynServer` with a new `Server`
    pub fn new() -> Self {
        Self {
            server: Server::new(),
        }
    }

    pub fn get<F>(&mut self, path: &str, handler: Box<F>) -> &mut Self
    where
        F: Fn(NgynRequest, NgynResponse) -> NgynResponse + Send + Sync + ?Sized + 'static,
    {
        self.route(path, HttpMethod::Get, handler)
    }

    pub fn post<F>(&mut self, path: &str, handler: Box<F>) -> &mut Self
    where
        F: Fn(NgynRequest, NgynResponse) -> NgynResponse + Send + Sync + ?Sized + 'static,
    {
        self.route(path, HttpMethod::Post, handler)
    }

    pub fn put<F>(&mut self, path: &str, handler: Box<F>) -> &mut Self
    where
        F: Fn(NgynRequest, NgynResponse) -> NgynResponse + Send + Sync + ?Sized + 'static,
    {
        self.route(path, HttpMethod::Put, handler)
    }

    pub fn delete<F>(&mut self, path: &str, handler: Box<F>) -> &mut Self
    where
        F: Fn(NgynRequest, NgynResponse) -> NgynResponse + Send + Sync + ?Sized + 'static,
    {
        self.route(path, HttpMethod::Delete, handler)
    }

    /// Adds a new route to the `NgynServer`.
    /// This function is chainable.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that represents the path of the route.
    /// * `method` - An `HttpMethod` that represents the HTTP method of the route.
    /// * `handler` - A closure that takes a `NgynRequest` and a `NgynResponse` and returns a `NgynResponse`.
    ///
    /// # Example
    ///
    /// ```
    /// let mut server = NgynServer::new();
    /// server.route("/", HttpMethod::Get, |req, res| {
    ///    res.status(200)
    /// });
    /// ```
    pub fn route<F>(&mut self, path: &str, method: HttpMethod, handler: Box<F>) -> &mut Self
    where
        F: Fn(NgynRequest, NgynResponse) -> NgynResponse + Send + Sync + ?Sized + 'static,
    {
        let handler = Arc::new(handler);
        let req_handler = {
            let handler = Arc::clone(&handler);
            move |req: tide::Request<()>| {
                let handler = Arc::clone(&handler);
                async move {
                    let request = NgynRequest::new(req);
                    let response = NgynResponse::new();
                    handler(request, response).build()
                }
            }
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
    pub async fn listen(self, address: &str) -> Result<()> {
        self.server.listen(address).await.map_err(tide::Error::from)
    }
}
