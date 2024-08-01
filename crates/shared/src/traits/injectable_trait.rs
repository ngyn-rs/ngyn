use std::any::Any;

use crate::server::NgynContext;

/// `NgynInjectable` is a trait that defines the basic structure of an injectable in Ngyn.
/// It is designed to be thread-safe.
pub trait NgynInjectable: AsAny + Send {
    /// Creates a new instance of the injectable.
    /// This is for internal use only.
    fn new() -> Self
    where
        Self: Sized;

    fn inject(&mut self, _cx: &NgynContext) {}
}

/// `AsAny` is a trait that allows a type to be converted to a trait object.
/// It is designed to be thread-safe.
pub trait AsAny: Any {
    /// Returns a reference to the trait object.
    fn as_any(&self) -> &dyn Any;
}

impl<T: NgynInjectable> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
