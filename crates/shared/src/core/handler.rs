use crate::server::{NgynContext, NgynResponse, ToBytes};

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

/// Creates a `Handler` trait object from a function that takes in a mutable reference to `NgynContext` and returns a type that implements `ToBytes`.
/// 
/// This function is useful for creating a `Handler` trait object from a function that returns any valid type that implements `ToBytes`.
///
/// ### Example
/// ```rust ignore
/// use ngyn::server::{handler, NgynContext, ToBytes};
///
/// app.get("/hello", handler(|ctx: &mut NgynContext| {
///    "Hello, World!"
/// }));
/// ```
pub fn handler<S: ToBytes + 'static>(
    f: impl Fn(&mut NgynContext) -> S + Send + Sync + 'static,
) -> Box<Handler> {
    Box::new(move |ctx: &mut NgynContext, res: &mut NgynResponse| {
        let body = f(ctx).to_bytes();
        *res.body_mut() = body.into();
    })
}
