use crate::NgynRequest;

/// Trait for implementing a gate.
pub trait NgynGate {
    /// Determines if the gate can activate for the given request.
    ///
    /// # Arguments
    ///
    /// * `request` - The request to check.
    ///
    /// # Returns
    ///
    /// Returns `true` if the route can activate, `false` otherwise.
    fn check(request: NgynRequest) -> bool;
}
