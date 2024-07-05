use crate::{
    server::{NgynContext, NgynResponse},
    traits::NgynInjectable,
};

/// Trait for implementing a gate.
pub trait NgynGate<'a>: NgynInjectable {
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
    fn can_activate(&self, _cx: &'a mut NgynContext, _res: &'a mut NgynResponse) -> bool {
        true
    }
}
