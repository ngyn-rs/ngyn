use std::any::Any;

use crate::server::NgynContext;

/// `NgynInjectable` is a trait that defines the basic structure of an injectable in Ngyn.
/// It is designed to be thread-safe.
pub trait NgynInjectable: Any + AsAny + AsAnyMut + Send {
    /// Creates a new instance of the injectable.
    /// This is for internal use only.
    fn new() -> Self
    where
        Self: Sized;

    fn inject(&mut self, _cx: &NgynContext) {}
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

pub trait AsAnyMut {
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: NgynInjectable> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<T: NgynInjectable> AsAnyMut for T {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
