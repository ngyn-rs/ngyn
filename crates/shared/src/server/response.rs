use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tide::{Response, Result};

use crate::{NgynController, NgynRequest};

struct NgynResponseRoute {
    controller: Arc<dyn NgynController>,
    handler: String,
    request: NgynRequest,
}

/// NgynResponse is a struct that represents a server response.
pub struct NgynResponse {
    route: Option<NgynResponseRoute>,
    pub status_code: u16,
    pub raw_body: String,
    pub headers: Vec<(String, String)>,
    pub cookies: Vec<(String, String)>,
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

    /// Sets the status code of the `NgynResponse`.
    ///
    /// # Arguments
    ///
    /// * `status` - A u16 that represents the status code to be set.
    ///
    /// # Returns
    ///
    /// * A mutable reference to the `NgynResponse`.
    pub fn status(mut self, status: u16) -> Self {
        self.status_code = status;
        self
    }

    /// Sets the body of the response
    ///
    /// # Arguments
    ///
    /// * `data` - A string that represents the body
    ///
    /// # Returns
    ///
    /// * A mutable reference to the `NgynResponse`.
    pub fn body(mut self, data: &str) -> Self {
        self.raw_body = data.to_string();
        self
    }

    // makes a clone of the response
    pub fn clone(&mut self) -> Self {
        Self {
            route: None,
            status_code: self.status_code,
            raw_body: self.raw_body.clone(),
            headers: self.headers.clone(),
            cookies: self.cookies.clone(),
        }
    }

    /// Builds the `NgynResponse`.
    pub fn build(self) -> Result {
        let response = Response::builder(self.status_code).body(self.raw_body);
        Ok(response.build())
    }

    /// Handles the `NgynResponse` from a route.
    pub fn from_route(
        mut self,
        controller: Arc<dyn NgynController>,
        handler: String,
        request: NgynRequest,
    ) -> Self {
        self.route = Some(NgynResponseRoute {
            controller,
            handler,
            request,
        });
        self
    }
}

impl Future for NgynResponse {
    type Output = NgynResponse;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.route.take() {
            Some(route) => {
                let handler = route.handler;
                let controller = route.controller;
                let request = route.request;
                let response = Self::default(); // TODO: add response fields to this

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
