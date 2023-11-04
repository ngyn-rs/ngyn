use crate::{NgynInjectable, NgynRequest};

/// Trait for implementing a gate.
pub trait NgynGate: NgynInjectable {
    /// Determines if the gate can activate for the given request.
    ///
    /// ### Arguments
    ///
    /// * `request` - The request to check.
    ///
    /// ### Returns
    ///
    /// Returns `true` if the route can activate, `false` otherwise.
    fn can_activate(self, request: &NgynRequest) -> bool;
}
