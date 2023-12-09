use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use crate::{NgynController, NgynRequest};

#[derive(Clone, PartialEq)]
pub enum NgynBody {
    String(String),
    Bool(bool),
    Number(usize),
    None,
}

impl From<String> for NgynBody {
    fn from(value: String) -> Self {
        NgynBody::String(value)
    }
}

impl From<bool> for NgynBody {
    fn from(value: bool) -> Self {
        NgynBody::Bool(value)
    }
}

impl From<usize> for NgynBody {
    fn from(value: usize) -> Self {
        NgynBody::Number(value)
    }
}

impl From<isize> for NgynBody {
    fn from(value: isize) -> Self {
        NgynBody::Number(value as usize)
    }
}

impl From<&str> for NgynBody {
    fn from(value: &str) -> Self {
        NgynBody::String(value.to_string())
    }
}

impl Into<NgynBody> for () {
    fn into(self) -> NgynBody {
        NgynBody::None
    }
}

#[derive(Clone)]
pub struct NgynResponseRoute {
    controller: Arc<dyn NgynController>,
    handler: String,
    request: NgynRequest,
}

/// NgynResponse is a struct that represents a server response.
#[derive(Clone)]
pub struct NgynResponse {
    status_code: u16,
    raw_body: NgynBody,
    raw_headers: Vec<String>,
    route: Option<NgynResponseRoute>,
}

impl NgynResponse {
    /// Constructs a new `NgynResponse` with a default status code of 200.
    pub fn new() -> Self {
        Self::from_status(200)
    }

    pub fn from_status(code: u16) -> Self {
        Self {
            status_code: code,
            raw_body: NgynBody::None,
            raw_headers: Vec::new(),
            route: None,
        }
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
    pub fn send(&mut self, data: &str) -> &mut Self {
        if self.raw_body == NgynBody::None {
            self.raw_body = NgynBody::String(data.to_string());
        } else {
            panic!("Response body already set");
        }
        self
    }

    /// Gets the raw value for response body
    pub fn body_raw(&self) -> NgynBody {
        self.raw_body.clone()
    }

    pub fn is_empty(&self) -> bool {
        match self.raw_body {
            NgynBody::String(ref value) => value.is_empty(),
            NgynBody::None => true,
            _ => false,
        }
    }

    /// Gets a header value by key
    pub fn header(&self, key: &str) -> Option<String> {
        self.raw_headers
            .iter()
            .find(|header| header.starts_with(key))
            .map(|header| header.split(":").nth(1).unwrap().trim().to_string())
    }

    pub fn headers(&self) -> Vec<String> {
        self.raw_headers.clone()
    }

    pub fn set_header(&mut self, key: &str, value: &str) -> &mut Self {
        self.raw_headers.push(format!("{}: {}", key, value));
        self
    }

    pub fn peek(&mut self, item: NgynBody) -> &mut Self {
        match item {
            NgynBody::String(value) => self.send(&value),
            NgynBody::Bool(value) => self.send(&value.to_string()),
            NgynBody::Number(value) => self.send(&value.to_string()),
            _ => self,
        }
    }

    /// Handles the `NgynResponse` from a route.
    pub fn with_controller(
        &mut self,
        controller: Arc<dyn NgynController>,
        handler: String,
        request: &NgynRequest,
    ) {
        self.route = Some(NgynResponseRoute {
            controller,
            handler,
            request: request.clone(),
        });
    }
}

impl Future for NgynResponse {
    type Output = NgynResponse;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.as_mut().get_mut() {
            NgynResponse { route, .. } => {
                let NgynResponseRoute {
                    controller,
                    handler,
                    request,
                } = route.take().unwrap();

                let mut response = self.clone();

                let _ = controller
                    .handle(handler, request, &mut response)
                    .as_mut()
                    .poll(cx);

                Poll::Ready(response)
            }
        }
    }
}

impl Default for NgynResponse {
    fn default() -> Self {
        Self::new()
    }
}
