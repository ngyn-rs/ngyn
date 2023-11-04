use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tide::{Response, Result};

use crate::{NgynController, NgynRequest};

#[derive(Clone)]
struct NgynResponseRoute {
    controller: Arc<dyn NgynController>,
    handler: String,
    request: NgynRequest,
}

/// NgynResponse is a struct that represents a server response.
pub struct NgynResponse {
    route: Option<NgynResponseRoute>,
    status_code: u16,
    raw_body: String,
    headers: Vec<(String, String)>,
    cookies: Vec<(String, String)>,
}

impl NgynResponse {
    /// Constructs a new `NgynResponse` with a default status code of 200.
    pub fn new() -> Self {
        Self {
            route: None,
            status_code: 200,
            raw_body: String::new(),
            headers: Vec::new(),
            cookies: Vec::new(),
        }
    }

    pub fn from_status(code: u16) -> Self {
        let mut response = Self::new();
        response.status_code = code;
        response
    }

    /// Sets the status code of the `NgynResponse`.
    ///
    /// ### Arguments
    ///
    /// * `status` - A u16 that represents the status code to be set.
    ///
    /// ### Returns
    ///
    /// * A mutable reference to the `NgynResponse`.
    pub fn set_status(&mut self, status: u16) -> &mut Self {
        self.status_code = status;
        self
    }

    /// Gets the status code of the response
    pub fn status(&self) -> u16 {
        self.status_code
    }

    /// Sets the body of the response
    ///
    /// ### Arguments
    ///
    /// * `data` - A string that represents the body
    ///
    /// ### Returns
    ///
    /// * A mutable reference to the `NgynResponse`.
    pub fn body(&mut self, data: &str) -> &mut Self {
        self.raw_body = data.to_string();
        self
    }

    /// Gets the raw value for response body
    pub fn raw(&self) -> String {
        self.raw_body.clone()
    }

    /// Builds the `NgynResponse`.
    pub fn build(self) -> Result {
        let response = Response::builder(self.status_code).body(self.raw_body);
        Ok(response.build())
    }

    /// Handles the `NgynResponse` from a route.
    pub fn with_controller(
        &mut self,
        controller: Arc<dyn NgynController>,
        handler: String,
        request: &NgynRequest,
    ) -> Self {
        self.route = Some(NgynResponseRoute {
            controller,
            handler,
            request: request.clone(),
        });
        self.clone()
    }
}

impl Clone for NgynResponse {
    fn clone(&self) -> Self {
        Self {
            route: self.route.clone(),
            status_code: self.status_code,
            raw_body: self.raw_body.clone(),
            headers: self.headers.clone(),
            cookies: self.cookies.clone(),
        }
    }
}

impl Future for NgynResponse {
    type Output = NgynResponse;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.route.take() {
            Some(NgynResponseRoute {
                handler,
                controller,
                request,
            }) => {
                let response = self.clone();

                let result = controller
                    .handle(handler, request, response)
                    .as_mut()
                    .poll(cx);

                match result {
                    Poll::Ready(result) => Poll::Ready(result),
                    Poll::Pending => Poll::Pending,
                }
            }
            None => Poll::Ready(self.clone()),
        }
    }
}

impl Default for NgynResponse {
    fn default() -> Self {
        Self::new()
    }
}
