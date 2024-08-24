use crate::{
    server::{NgynContext, NgynResponse},
    traits::NgynInjectable,
};

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
/// use ngyn_shared::traits::{NgynGate, NgynInjectable};
/// use ngyn_shared::server::{NgynContext, NgynResponse};
///
/// pub struct AuthGate {}
///
/// impl NgynInjectable for AuthGate {
///   fn new() -> Self {
///     AuthGate {}
///   }
/// }
///
/// impl NgynGate for AuthGate {
///    async fn can_activate(&self, cx: &mut NgynContext, res: &mut NgynResponse) -> bool {
///      // Check if the user is authenticated
///      // If the user is authenticated, return true
///      // Otherwise, return false
///       false
///     }
/// }
/// ```
pub trait NgynGate: NgynInjectable {
    /// Determines if the gate can activate for the given request.
    ///
    /// ### Arguments
    ///
    /// * `cx` - The request context to check.
    /// * `res` - The response to optionally modify.
    ///
    /// ### Returns
    ///
    /// Returns `true` if the route can activate, `false` otherwise.
    #[allow(async_fn_in_trait, unused_variables)]
    async fn can_activate(&self, cx: &mut NgynContext, res: &mut NgynResponse) -> bool {
        true // default implementation
    }
}
