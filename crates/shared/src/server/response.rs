use std::sync::Arc;

use http_body_util::Full;
use hyper::{body::Bytes, Method, Request, Response, StatusCode};

use crate::{context::NgynContext, Handler, NgynResponse, ToBytes};

/// Trait representing a full response.
pub trait FullResponse {
    /// Sets the status code of the response.
    ///
    /// # Arguments
    ///
    /// * `status` - The status code to set.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use http_body_util::Full;
    /// use hyper::StatusCode;
    /// use crate::{context::NgynContext, transformer::Transformer, NgynResponse, ToBytes};
    ///
    /// struct MyResponse {
    ///     status: StatusCode,
    /// }
    ///
    /// let mut response = MyResponse { status: StatusCode::OK };
    /// response.set_status(200);
    /// assert_eq!(response.status, StatusCode::OK);
    /// ```
    fn set_status(&mut self, status: u16);

    /// Sends the body of the response.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to parse the body from.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use http_body_util::Full;
    /// use hyper::StatusCode;
    /// use crate::{context::NgynContext, transformer::Transformer, NgynResponse, ToBytes};
    ///
    /// struct MyResponse {
    ///     body: Full,
    /// }
    ///
    /// let mut response = MyResponse { body: Full::new(vec![1, 2, 3]) };
    /// response.send(vec![4, 5, 6]);
    /// assert_eq!(response.body.as_slice(), &[4, 5, 6]);
    /// ```
    fn send(&mut self, item: impl ToBytes);
}

impl FullResponse for NgynResponse {
    fn set_status(&mut self, status: u16) {
        *self.status_mut() = StatusCode::from_u16(status).unwrap();
    }

    fn send(&mut self, item: impl ToBytes) {
        *self.body_mut() = Full::new(item.to_bytes());
    }
}

#[async_trait::async_trait]
pub trait ResponseBuilder: FullResponse {
    /// Creates a new response.
    ///
    /// # Arguments
    ///
    /// * `req` - The request to create the response from.
    ///
    /// # Returns
    ///
    /// A new response.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use http_body_util::Full;
    /// use hyper::StatusCode;
    /// use crate::{context::NgynContext, transformer::Transformer, NgynResponse, ToBytes};
    ///
    /// let response = NgynResponse::init(req, routes);
    /// assert_eq!(response.status, StatusCode::OK);
    /// assert_eq!(response.body.as_slice(), &[1, 2, 3]);
    /// ```
    async fn init(
        req: Request<Vec<u8>>,
        routes: Arc<Vec<(String, Method, Box<Handler>)>>,
        middlewares: Arc<Vec<Box<dyn crate::NgynMiddleware>>>,
    ) -> Self;
}

#[async_trait::async_trait]
impl ResponseBuilder for NgynResponse {
    async fn init(
        req: Request<Vec<u8>>,
        routes: Arc<Vec<(String, Method, Box<Handler>)>>,
        middlewares: Arc<Vec<Box<dyn crate::NgynMiddleware>>>,
    ) -> Self {
        let mut cx = NgynContext::from_request(req);
        let mut res = Response::new(Full::new(Bytes::default()));

        let handler = routes
            .iter()
            .filter_map(|(path, method, handler)| {
                if cx.with(path, method).is_some() {
                    Some(handler)
                } else {
                    None
                }
            })
            .next();

        middlewares
            .iter()
            .for_each(|middleware| middleware.handle(&mut cx, &mut res));

        if let Some(handler) = handler {
            handler(&mut cx, &mut res);
            cx.execute(&mut res).await;
        }

        res
    }
}
