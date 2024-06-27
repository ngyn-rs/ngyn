use std::sync::{Arc, Mutex};

use http_body_util::Full;
use hyper::{body::Bytes, header::IntoHeaderName, Method, Request, Response, StatusCode};

use crate::{
    core::Handler,
    server::{NgynContext, NgynResponse, ToBytes, Transformer},
};

use super::context::AppState;

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

    /// Sets a header - name, value pair - of the response.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the header.
    /// * `value` - The value of the header.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// let mut response = NgynResponse::default();
    /// response.set_header("Content-Type", "application/json");
    /// assert_eq!(response.headers.get("Content-Type"), Some(&"application/json".to_string()));
    /// ```
    fn set_header<K: IntoHeaderName>(&mut self, name: K, value: &str);

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

    fn set_header<K: IntoHeaderName>(&mut self, name: K, value: &str) {
        self.headers_mut().insert(name, value.parse().unwrap());
    }

    fn send(&mut self, item: impl ToBytes) {
        *self.body_mut() = Full::new(item.to_bytes());
    }
}

impl<'a> Transformer<'a> for &'a NgynResponse {
    fn transform(_cx: &'a mut NgynContext, res: &'a mut NgynResponse) -> Self {
        res
    }
}

impl<'a> Transformer<'a> for &'a mut NgynResponse {
    fn transform(_cx: &'a mut NgynContext, res: &'a mut NgynResponse) -> Self {
        res
    }
}

pub(crate) type Routes = Vec<(String, Method, Box<Handler>)>;
pub(crate) type Middlewares = Vec<Box<dyn crate::traits::NgynMiddleware>>;

#[async_trait::async_trait]
pub(crate) trait ResponseBuilder: FullResponse {
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
    /// let response = NgynResponse::build(req, routes);
    /// assert_eq!(response.status, StatusCode::OK);
    /// assert_eq!(response.body.as_slice(), &[1, 2, 3]);
    /// ```
    async fn build(
        req: Request<Vec<u8>>,
        routes: Arc<Mutex<Routes>>,
        middlewares: Arc<Mutex<Middlewares>>,
    ) -> Self;

    async fn build_with_state<T: AppState>(
        req: Request<Vec<u8>>,
        routes: Arc<Mutex<Routes>>,
        middlewares: Arc<Mutex<Middlewares>>,
        state: T,
    ) -> Self;
}

#[async_trait::async_trait]
impl ResponseBuilder for NgynResponse {
    async fn build(
        req: Request<Vec<u8>>,
        routes: Arc<Mutex<Vec<(String, Method, Box<Handler>)>>>,
        middlewares: Arc<Mutex<Vec<Box<dyn crate::traits::NgynMiddleware>>>>,
    ) -> Self {
        Self::build_with_state(req, routes, middlewares, ()).await
    }

    async fn build_with_state<T: AppState>(
        req: Request<Vec<u8>>,
        routes: Arc<Mutex<Vec<(String, Method, Box<Handler>)>>>,
        middlewares: Arc<Mutex<Vec<Box<dyn crate::traits::NgynMiddleware>>>>,
        state: T,
    ) -> Self {
        let mut cx = NgynContext::from_request(req);
        let mut res = Response::new(Full::new(Bytes::default()));

        cx.set_state(Box::new(state));

        let mut is_handled = false;
        let _ = &routes
            .lock()
            .unwrap()
            .iter()
            .for_each(|(path, method, route_handler)| {
                if !is_handled && cx.with(path, method).is_some() {
                    middlewares
                        .lock()
                        .unwrap()
                        .iter()
                        .for_each(|middlewares| middlewares.handle(&mut cx, &mut res));
                    route_handler(&mut cx, &mut res);
                    is_handled = true;
                }
            });

        if is_handled {
            cx.execute(&mut res).await;
        }

        res
    }
}
