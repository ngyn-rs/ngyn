use ngyn_shared::{HttpMethod, NgynRequest, NgynResponse};
use std::sync::Arc;
use tide::{Result, Server};

use super::{Handler, NgynEngine};

/// `NgynService` is a struct that represents a server instance in the Ngyn framework.
pub struct NgynService {
    server: Server<()>,
}

impl NgynEngine for NgynService {
    fn new() -> Self {
        Self {
            server: Server::new(),
        }
    }

    fn route(&mut self, path: &str, method: HttpMethod, handler: Box<impl Handler>) -> &mut Self {
        let handler = Arc::new(handler);
        let req_handler = {
            let handler = Arc::clone(&handler);
            move |req: tide::Request<()>| {
                let handler = Arc::clone(&handler);
                async move {
                    let request = NgynRequest::from(req);
                    let response = NgynResponse::new();
                    handler.handle(request, response).await.build()
                }
            }
        };
        match method {
            HttpMethod::Get => self.server.at(path).get(req_handler),
            HttpMethod::Post => self.server.at(path).post(req_handler),
            HttpMethod::Put => self.server.at(path).put(req_handler),
            HttpMethod::Delete => self.server.at(path).delete(req_handler),
            HttpMethod::Patch => self.server.at(path).patch(req_handler),
            HttpMethod::Head => self.server.at(path).head(req_handler),
            _ => panic!("Unsupported HTTP method"),
        };
        self
    }
}

impl NgynService {
    /// Adds a new route to the `NgynService` with the `HttpMethod::Get`.
    pub fn get(&mut self, path: &str, handler: impl Handler) -> &mut Self {
        self.route(path, HttpMethod::Get, Box::new(handler))
    }

    /// Adds a new route to the `NgynService` with the `HttpMethod::Post`.
    pub fn post(&mut self, path: &str, handler: impl Handler) -> &mut Self {
        self.route(path, HttpMethod::Get, Box::new(handler))
    }

    /// Adds a new route to the `NgynService` with the `HttpMethod::Put`.
    pub fn put(&mut self, path: &str, handler: impl Handler) -> &mut Self {
        self.route(path, HttpMethod::Get, Box::new(handler))
    }

    /// Adds a new route to the `NgynService` with the `HttpMethod::Delete`.
    pub fn delete(&mut self, path: &str, handler: impl Handler) -> &mut Self {
        self.route(path, HttpMethod::Get, Box::new(handler))
    }

    /// Adds a new route to the `NgynService` with the `HttpMethod::Patch`.
    pub fn patch(&mut self, path: &str, handler: impl Handler) -> &mut Self {
        self.route(path, HttpMethod::Get, Box::new(handler))
    }

    /// Adds a new route to the `NgynService` with the `HttpMethod::Head`.
    pub fn head(&mut self, path: &str, handler: impl Handler) -> &mut Self {
        self.route(path, HttpMethod::Get, Box::new(handler))
    }

    /// Starts listening for incoming connections on the specified address.
    /// This function is asynchronous and returns a `tide::Result`.
    pub async fn listen(self, address: &str) -> Result<()> {
        self.server.listen(address).await.map_err(tide::Error::from)
    }
}

impl Default for NgynService {
    fn default() -> Self {
        Self::new()
    }
}
