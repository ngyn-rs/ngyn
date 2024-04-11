use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use crate::{body::ParseBytes, transformer::Transformer, Bytes, NgynController, NgynRequest, ParseBody};

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
    raw_body: Bytes,
    raw_headers: Vec<String>,
    route: Option<NgynResponseRoute>,
}

impl NgynResponse {
    pub fn from_status(code: u16) -> Self {
        Self {
            status_code: code,
            raw_body: Bytes::default(),
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
        if self.raw_body.is_empty() {
            self.raw_body = Bytes::from(data.to_string());
        } else {
            panic!("Response body already set");
        }
        self
    }

    /// Gets the raw value for response body
    pub fn body_raw(&self) -> Bytes {
        self.raw_body.clone()
    }

    pub fn is_empty(&self) -> bool {
        self.raw_body.is_empty()
    }

    /// Gets a header value by key
    pub fn header(&self, key: &str) -> Option<String> {
        self.raw_headers
            .iter()
            .find(|header| header.starts_with(key))
            .map(|header| header.split(':').nth(1).unwrap().trim().to_string())
    }

    pub fn headers(&self) -> Vec<String> {
        self.raw_headers.clone()
    }

    pub fn set_header(&mut self, key: &str, value: &str) -> &mut Self {
        self.raw_headers.push(format!("{}: {}", key, value));
        self
    }

    pub fn peek(&mut self, item: impl ParseBody) -> &mut Self {
        self.send(&item.parse_body().parse_bytes::<String>())
    }

    /// Handles the `NgynResponse` from a route.
    pub fn with_controller(
        &mut self,
        controller: Arc<dyn NgynController>,
        handler: String,
        request: &mut NgynRequest,
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
        let NgynResponse { route, .. } = self.as_mut().get_mut();

        if let Some(NgynResponseRoute {
            controller,
            handler,
            mut request,
        }) = route.clone()
        {
            let mut response = self.clone();

            let _ = controller
                .handle(&handler, &mut request, &mut response)
                .as_mut()
                .poll(cx);

            Poll::Ready(response)
        } else {
            Poll::Ready(self.clone())
        }
    }
}

impl Transformer for NgynResponse {
    fn transform(_req: &mut NgynRequest, res: &mut NgynResponse) -> Self {
        res.clone()
    }
}
