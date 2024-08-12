use crate::traits::NgynController;
use std::sync::Arc;

/// Modules are the building blocks of an application in Ngyn.
/// They are used to group related [controllers](https://ngyn.rs/docs/foundations/controllers).
///
/// ### Example
///
/// ```rust
/// use ngyn_shared::traits::NgynModule;
///
/// pub struct AppModule;
///
/// impl NgynModule for AppModule {
///    fn new() -> Self {
///       Self {}
///   }
/// }
/// ```
pub trait NgynModule: Send + Sync {
    /// Creates a new instance of the module.
    fn new() -> Self
    where
        Self: Sized;

    /// Returns the controllers of the module.
    fn get_controllers(&self) -> Vec<Arc<Box<dyn NgynController + 'static>>> {
        Vec::new()
    }
}
