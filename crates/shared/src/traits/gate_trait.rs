use crate::{NgynInjectable, NgynContext};

/// Trait for implementing a gate.
pub trait NgynGate: NgynInjectable {
    /// Determines if the gate can activate for the given request.
    ///
    /// ### Arguments
    ///
    /// * `cx` - The request context to check.
    ///
    /// ### Returns
    ///
    /// Returns `true` if the route can activate, `false` otherwise.
    fn can_activate(self, cx: &mut NgynContext) -> bool;
}
