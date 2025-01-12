use std::{future::Future, pin::Pin};

use http::{HeaderValue, StatusCode};

use crate::server::{NgynContext, ToBytes};

/// Represents a handler function that takes in a mutable reference to `NgynContext` and `NgynResponse`.
pub(crate) type Handler = dyn Fn(&mut NgynContext) -> Box<dyn ToBytes> + Send + Sync + 'static;

pub(crate) type AsyncHandler = dyn for<'a, 'b> Fn(
        &'a mut NgynContext,
    ) -> Pin<Box<dyn Future<Output = Box<dyn ToBytes>> + Send + 'a>>
    + Send
    + Sync;

pub enum RouteHandler {
    Sync(Box<Handler>),
    Async(Box<AsyncHandler>),
}

impl RouteHandler {
    pub fn from_async(
        f: impl for<'a, 'b> Fn(
                &'a mut NgynContext,
            )
                -> Pin<Box<dyn Future<Output = Box<dyn ToBytes>> + Send + 'a>>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        RouteHandler::Async(Box::new(f))
    }
}

impl From<Box<AsyncHandler>> for RouteHandler {
    fn from(f: Box<AsyncHandler>) -> Self {
        RouteHandler::Async(f)
    }
}

impl<F: Fn(&mut NgynContext) -> Box<dyn ToBytes> + Send + Sync + 'static> From<F> for RouteHandler {
    fn from(f: F) -> Self {
        RouteHandler::Sync(Box::new(f))
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
    Box::new(move |ctx: &mut NgynContext| {
        let body = f(ctx);
        Box::new(body) as Box<dyn ToBytes>
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
) -> Box<AsyncHandler> {
    Box::new(move |ctx: &mut NgynContext| {
        let fut = f(ctx);
        Box::pin(async move { Box::new(fut.await) as Box<dyn ToBytes> })
    })
}

pub fn async_wrap(
    f: impl for<'a, 'b> Fn(
            &'a mut NgynContext,
        ) -> Pin<Box<dyn Future<Output = Box<dyn ToBytes>> + Send + 'a>>
        + Send
        + Sync
        + 'static,
) -> Box<AsyncHandler> {
    Box::new(f)
}

/// Create a not-implemented handler that returns a `501 Not Implemented` status code.
///
/// This is very similar to unimplemented! macro in Rust.
pub fn not_implemented() -> Box<Handler> {
    Box::new(|ctx: &mut NgynContext| {
        *ctx.response_mut().status_mut() = StatusCode::NOT_IMPLEMENTED;
        Box::new(()) as Box<dyn ToBytes>
    })
}

/// Redirects to a specified location with a `303 See Other` status code.
pub fn redirect_to(location: &'static str) -> Box<Handler> {
    Box::new(|ctx: &mut NgynContext| {
        ctx.response_mut()
            .headers_mut()
            .insert("Location", HeaderValue::from_str(location).unwrap());
        *ctx.response_mut().status_mut() = StatusCode::SEE_OTHER;
        Box::new(()) as Box<dyn ToBytes>
    })
}

/// Redirects to a specified location with a `307 Temporary Redirect` status code.
pub fn redirect_temporary(location: &'static str) -> Box<Handler> {
    Box::new(|ctx: &mut NgynContext| {
        ctx.response_mut()
            .headers_mut()
            .insert("Location", HeaderValue::from_str(location).unwrap());
        *ctx.response_mut().status_mut() = StatusCode::TEMPORARY_REDIRECT;
        Box::new(()) as Box<dyn ToBytes>
    })
}

/// Redirects to a specified location with a `301 Moved Permanently` status code.
pub fn redirect_permanent(location: &'static str) -> Box<Handler> {
    Box::new(|ctx: &mut NgynContext| {
        ctx.response_mut()
            .headers_mut()
            .insert("Location", HeaderValue::from_str(location).unwrap());
        *ctx.response_mut().status_mut() = StatusCode::MOVED_PERMANENTLY;
        Box::new(()) as Box<dyn ToBytes>
    })
}

/// Redirects to a specified location with a `302 Found` status code.
pub fn redirect_found(location: &'static str) -> Box<Handler> {
    Box::new(|ctx: &mut NgynContext| {
        ctx.response_mut()
            .headers_mut()
            .insert("Location", HeaderValue::from_str(location).unwrap());
        *ctx.response_mut().status_mut() = StatusCode::FOUND;
        Box::new(()) as Box<dyn ToBytes>
    })
}
