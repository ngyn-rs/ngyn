use std::{future::Future, pin::Pin};

use crate::server::{NgynContext, NgynResponse};

/// Trait for implementing a middleware.
///
/// Middlewares are how Ngyn processes requests.
/// They can be used to modify the request context, the response, or both.
///
/// A few things to note about middlewares:
/// - They are executed in the order they are added.
/// - They can be used to modify the request context, the response, or both.
/// - They can be used to short-circuit the request handling process.
/// - They are purely synchronous and should not ideally not have side effects.
///
/// ### Examples
///
/// ```rust
/// use ngyn_shared::traits::{NgynMiddleware, NgynInjectable};
/// use ngyn_shared::server::{NgynContext, NgynResponse};
///
/// pub struct RequestReceivedLogger {}
///
/// impl NgynInjectable for RequestReceivedLogger {
///    fn new() -> Self {
///       RequestReceivedLogger {}
///   }
/// }
///
/// impl NgynMiddleware for RequestReceivedLogger {
///   async fn handle(&self, cx: &mut NgynContext, res: &mut NgynResponse) {
///    println!("Request received: {:?}", cx.request());
///  }
/// }
/// ```
pub trait NgynMiddleware: Send + Sync {
    /// Handles the request.
    #[allow(async_fn_in_trait)]
    fn handle<'a>(
        cx: &'a mut NgynContext,
        res: &'a mut NgynResponse,
    ) -> impl std::future::Future<Output = ()> + Send + 'a
    where
        Self: Sized;
}

pub(crate) trait Middleware: Send + Sync {
    fn run<'a>(
        &'a self,
        _cx: &'a mut NgynContext,
        _res: &'a mut NgynResponse,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async move {})
    }
}

impl<T: NgynMiddleware + Send> Middleware for T {
    fn run<'a>(
        &'a self,
        cx: &'a mut NgynContext,
        res: &'a mut NgynResponse,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(T::handle(cx, res))
    }
}
