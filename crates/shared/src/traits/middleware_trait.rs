use crate::{
    server::{NgynContext, NgynResponse},
    traits::NgynInjectable,
};

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
/// # Examples
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
///   fn handle(&self, cx: &mut NgynContext, res: &mut NgynResponse) {
///    println!("Request received: {:?}", cx.request());
///  }
/// }
/// ```
pub trait NgynMiddleware: NgynInjectable + Sync {
    /// Handles the request.
    fn handle(&self, cx: &mut NgynContext, res: &mut NgynResponse);
}
