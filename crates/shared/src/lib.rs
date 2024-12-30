pub mod core;
pub mod server;

use std::{future::Future, pin::Pin};

use server::context::NgynContext;

/// Trait to configure a gate, middleware or related service.
///
/// It is useful when you need to pass configuration to a gate, middleware or related service.
///
/// ### Example
///
/// ```rust
/// # use ngyn_shared::WithConfig;
/// # use ngyn_shared::server::NgynContext;
/// # use ngyn_shared::NgynGate;
///
/// struct AuthGateConfig {
///    pub secret: String,
/// }
///
/// impl Default for AuthGateConfig {
///   fn default() -> Self {
///    Self {
///     secret: "my_secret".to_string(),
///    }
///   }
/// }
///
/// struct AuthGate;
/// 
/// impl WithConfig<AuthGateConfig> for AuthGate {}
///
/// impl NgynGate for AuthGate {
///   async fn can_activate(cx: &mut NgynContext<'_>) -> bool {
///    let config = Self::config();
///    config.secret == "my_secret"
///   }
/// }
/// ```
pub trait WithConfig<C: Default> {
    fn config() -> C {
        Default::default()
    }
}

/// Trait for implementing a gate.
///
/// Gates are how Ngyn determines if a route can activate.
/// Sometimes, a route may need to be guarded by certain conditions.
/// For instance, restricting access to a route based on the user's role, or checking if the user is authenticated.
/// Typically, gates are used for this purpose.
///
/// ### Examples
///
/// ```rust
/// # use ngyn_shared::NgynGate;
/// # use ngyn_shared::server::NgynContext;
///
/// struct AuthGate;
///
/// impl NgynGate for AuthGate {
///    async fn can_activate(cx: &mut NgynContext<'_>) -> bool {
///      // Check if the user is authenticated
///      // If the user is authenticated, return true
///      // Otherwise, return false
///       false
///     }
/// }
/// ```
pub trait NgynGate {
    /// Determines if the gate can activate for the given request.
    ///
    /// ### Arguments
    ///
    /// * `cx` - The request context to check.
    ///
    /// ### Returns
    ///
    /// Returns `true` if the route can activate, `false` otherwise.
    #[allow(async_fn_in_trait, unused_variables)]
    async fn can_activate(cx: &mut NgynContext<'_>) -> bool {
        true // default implementation
    }
}

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
/// # use ngyn_shared::NgynMiddleware;
/// # use ngyn_shared::server::NgynContext;
///
/// pub struct RequestReceivedLogger {}
///
/// impl NgynMiddleware for RequestReceivedLogger {
///   async fn handle(cx: &mut NgynContext<'_>) {
///    println!("Request received: {:?}", cx.request());
///  }
/// }
/// ```
pub trait NgynMiddleware: Send + Sync {
    /// Handles the request.
    #[allow(async_fn_in_trait)]
    fn handle(cx: &mut NgynContext<'_>) -> impl std::future::Future<Output = ()> + Send
    where
        Self: Sized;
}

pub(crate) trait Middleware: Send + Sync {
    fn run<'a>(
        &'a self,
        _cx: &'a mut NgynContext<'_>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;
}

impl<'b, T: NgynMiddleware + Send + 'b> Middleware for T {
    fn run<'a>(
        &'a self,
        cx: &'a mut NgynContext<'_>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(T::handle(cx))
    }
}
