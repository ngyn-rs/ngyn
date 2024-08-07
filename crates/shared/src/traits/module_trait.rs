use crate::traits::NgynController;
use std::sync::Arc;

/// Modules are the building blocks of an application in Ngyn.
/// They are used to group related [controllers](https://ngyn.rs/docs/foundations/controllers).
///
/// ### Example
///
/// ```rust
/// use ngyn::prelude::*;
///
/// #[module]
/// pub struct AppModule;
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
