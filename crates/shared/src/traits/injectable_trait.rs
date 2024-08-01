use std::any::Any;

use crate::server::NgynContext;

/// `NgynInjectable` is a thread-safe trait that defines the basic structure of an injectable in Ngyn.
///
/// Injectables (not to be confused with dependency injection) are used to define the structure of a component in Ngyn.
/// They have varying uses from middleware, to gates, guards, services, controllers, and more.
///
/// They are solely called `injectables` because they have a method called `inject` that is used to inject the context into the component.
///
/// More information on injectables can be found in the [docs](https://ngyn.rs/docs/foundations/injectables/).
///
/// ### Example
///
/// ```rust
/// use ngyn_shared::traits::NgynInjectable;
///
/// #[derive(Default)]
/// pub struct MyInjectable;
///
/// impl NgynInjectable for MyInjectable {
///    fn new() -> Self {
///       MyInjectable {}
///   }
/// }
/// ```
pub trait NgynInjectable: AsAny + Send {
    /// Creates a new instance of the injectable.
    /// This is for internal use only.
    fn new() -> Self
    where
        Self: Sized;

    fn inject(&mut self, _cx: &NgynContext) {}
}

/// `AsAny` is an extra thread-safe trait implemented by all injectables.
///
/// Its use is to allow for downcasting of injectables to specific types.
///
/// More information on downcasting can be found in the [docs](https://ngyn.rs/docs/foundations/injectables/#downcasting).
///
/// ### Example
///
/// ```rust
/// use ngyn_shared::traits::AsAny;
/// use ngyn_shared::traits::NgynInjectable;
///
/// #[derive(Default)]
/// pub struct MyInjectable;
///
/// impl NgynInjectable for MyInjectable {
///    fn new() -> Self {
///       MyInjectable {}
///   }
/// }
///
/// let injectable = MyInjectable::new();
/// let injectable_any = injectable.as_any();
/// let my_injectable = injectable_any.downcast_ref::<MyInjectable>().unwrap();
/// ```
pub trait AsAny: Any {
    /// Returns a reference to the trait object.
    fn as_any(&self) -> &dyn Any;
}

impl<T: NgynInjectable> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
