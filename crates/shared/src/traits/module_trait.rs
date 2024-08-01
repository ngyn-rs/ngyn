use crate::traits::NgynController;
use std::sync::Arc;

/// `NgynModule` is a trait that defines the basic structure of a module in Ngyn.
pub trait NgynModule: Send + Sync {
    /// Creates a new instance of the module.
    fn new() -> Self
    where
        Self: Sized;

    /// Returns the name of the module.
    fn name(&self) -> &str;

    /// Returns the controllers of the module.
    fn get_controllers(&self) -> Vec<Arc<Box<dyn NgynController + 'static>>> {
        vec![]
    }
}
