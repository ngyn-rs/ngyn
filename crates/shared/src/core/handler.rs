use crate::{NgynContext, NgynResponse};

/// Represents a handler function that takes in a mutable reference to `NgynContext` and `NgynResponse`.
pub type Handler = dyn Fn(&mut NgynContext, &mut NgynResponse) + Send + Sync + 'static;

/// Represents a trait for converting a type into a `Handler` trait object.
pub trait RouteHandle: Send + Sync {
    /// Converts the implementing type into a `Handler` trait object.
    fn into(self) -> Box<Handler>;
}

impl<F> RouteHandle for F
where
    F: Fn(&mut NgynContext, &mut NgynResponse) + Send + Sync + 'static,
{
    /// Converts the implementing function into a `Handler` trait object.
    fn into(self) -> Box<Handler> {
        Box::new(self)
    }
}
