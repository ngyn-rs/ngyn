use std::{future::Future, pin::Pin};

use http::{HeaderValue, StatusCode};

use crate::server::{NgynContext, NgynResponse, ToBytes};

/// Represents a handler function that takes in a mutable reference to `NgynContext` and `NgynResponse`.
pub(crate) type Handler = dyn Fn(&mut NgynContext, &mut NgynResponse) + Send + Sync + 'static;

pub(crate) type AsyncHandler = Box<
    dyn for<'a, 'b> Fn(
            &'a mut NgynContext,
            &'b mut NgynResponse,
        ) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
        + Send
        + Sync,
>;

type AsyncHandlerFn = dyn for<'a, 'b> Fn(
        &'a mut NgynContext,
        &'b mut NgynResponse,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    + Send
    + Sync;

pub enum RouteHandler {
    Sync(Box<Handler>),
    Async(AsyncHandler),
}

impl<F: Fn(&mut NgynContext, &mut NgynResponse) + Send + Sync + 'static> From<F> for RouteHandler {
    fn from(f: F) -> Self {
        RouteHandler::Sync(Box::new(f))
    }
}

impl From<AsyncHandler> for RouteHandler {
    fn from(f: AsyncHandler) -> Self {
        RouteHandler::Async(f)
    }
}

/// Creates a `Handler` trait object from a function that takes in a mutable reference to `NgynContext` and returns a type that implements `ToBytes`.
///
/// This function is useful for creating a `Handler` trait object from a function that returns any valid type that implements `ToBytes`.
///
/// ### Example
/// ```rust ignore
/// use ngyn::server::{handler, NgynContext, ToBytes};
///
/// app.get("/hello", handler(|ctx: &mut NgynContext| {
///    "Hello, World!"
/// }));
/// ```
pub fn handler<S: ToBytes + 'static>(
    f: impl Fn(&mut NgynContext) -> S + Send + Sync + 'static,
) -> Box<Handler> {
    Box::new(move |ctx: &mut NgynContext, res: &mut NgynResponse| {
        let body = f(ctx).to_bytes();
        *res.body_mut() = body.into();
    })
}

/// Creates a `AsyncHandler` trait object from an async function that takes in a mutable reference to `NgynContext` and returns a future with output that implements `ToBytes`.
///
/// ### Example
/// ```rust ignore
/// use ngyn::server::{async_handler, NgynContext, ToBytes};
///
/// app.get("/hello", async_handler(|ctx: &mut NgynContext| async {
///    "Hello, World!"
/// }));
/// ```
pub fn async_handler<S: ToBytes + 'static, Fut: Future<Output = S> + Send + 'static>(
    f: impl Fn(&mut NgynContext) -> Fut + Send + Sync + 'static,
) -> AsyncHandler {
    Box::new(move |ctx: &mut NgynContext, res: &mut NgynResponse| {
        let fut = f(ctx);
        Box::pin(async move {
            let body = fut.await.to_bytes();
            *res.body_mut() = body.into();
        })
    })
}

/// Create a not-implemented handler that returns a `501 Not Implemented` status code.
///
/// This is very similar to unimplemented! macro in Rust.
pub fn not_implemented() -> Box<Handler> {
    Box::new(|_ctx: &mut NgynContext, res: &mut NgynResponse| {
        *res.status_mut() = StatusCode::NOT_IMPLEMENTED;
    })
}

/// Redirects to a specified location with a `303 See Other` status code.
pub fn redirect_to(location: &'static str) -> Box<Handler> {
    Box::new(|_ctx: &mut NgynContext, res: &mut NgynResponse| {
        res.headers_mut()
            .insert("Location", HeaderValue::from_str(location).unwrap());
        *res.status_mut() = StatusCode::SEE_OTHER;
    })
}

/// Redirects to a specified location with a `307 Temporary Redirect` status code.
pub fn redirect_temporary(location: &'static str) -> Box<Handler> {
    Box::new(|_ctx: &mut NgynContext, res: &mut NgynResponse| {
        res.headers_mut()
            .insert("Location", HeaderValue::from_str(location).unwrap());
        *res.status_mut() = StatusCode::TEMPORARY_REDIRECT;
    })
}

/// Redirects to a specified location with a `301 Moved Permanently` status code.
pub fn redirect_permanent(location: &'static str) -> Box<Handler> {
    Box::new(|_ctx: &mut NgynContext, res: &mut NgynResponse| {
        res.headers_mut()
            .insert("Location", HeaderValue::from_str(location).unwrap());
        *res.status_mut() = StatusCode::MOVED_PERMANENTLY;
    })
}

/// Redirects to a specified location with a `302 Found` status code.
pub fn redirect_found(location: &'static str) -> Box<Handler> {
    Box::new(|_ctx: &mut NgynContext, res: &mut NgynResponse| {
        res.headers_mut()
            .insert("Location", HeaderValue::from_str(location).unwrap());
        *res.status_mut() = StatusCode::FOUND;
    })
}
