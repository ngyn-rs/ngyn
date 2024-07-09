use std::any::Any;

use crate::server::NgynContext;

/// `NgynInjectable` is a trait that defines the basic structure of an injectable in Ngyn.
/// It is designed to be thread-safe.
pub trait NgynInjectable: Any + Send {
    /// Creates a new instance of the injectable.
    /// This is for internal use only.
    fn new() -> Self
    where
        Self: Sized;

    fn inject(&mut self, _cx: &NgynContext) {}

    fn as_any(&self) -> &dyn Any
    where
        Self: Sized,
    {
        self
    }
}
