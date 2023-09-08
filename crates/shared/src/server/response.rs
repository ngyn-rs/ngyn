use std::{
    convert::TryFrom,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tide::{Response, Result, StatusCode};

use crate::{NgynController, NgynRequest};

struct NgynResponseRoute {
    controller: Arc<dyn NgynController>,
    handler: String,
    request: NgynRequest,
}

/// NgynResponse is a struct that represents a server response.
pub struct NgynResponse {
    response: Response,
    route: Option<NgynResponseRoute>,
}

impl NgynResponse {
    /// Constructs a new `NgynResponse` with a default status code of 200.
    pub fn new() -> Self {
        Self {
            response: Response::new(200),
            route: None,
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
        self.response
            .set_status(StatusCode::try_from(status).unwrap());
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
        self.response.set_body(data);
        self
    }

    // makes a clone of the response
    pub fn clone(&mut self) -> Self {
        let mut response = Response::from(self.response.take_body());
        response.set_status(self.response.status());

        Self {
            response,
            route: None,
        }
    }

    /// Builds the `NgynResponse`.
    pub fn build(self) -> Result {
        Ok(self.response)
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

impl From<Response> for NgynResponse {
    fn from(response: Response) -> Self {
        Self {
            response,
            route: None,
        }
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
                let response = Self::default();

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
