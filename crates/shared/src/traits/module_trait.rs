use crate::{NgynController, NgynMiddleware};

/// `NgynModule` is a trait that defines the basic structure of a module in Ngyn.
pub trait NgynModule: Send + Sync {
    /// Creates a new instance of the module.
    fn new() -> Self
    where
        Self: Sized;

    /// Returns the name of the module.
    fn name(&self) -> &str;

    /// Returns the controllers of the module.
    fn get_controllers(&self) -> Vec<std::sync::Arc<dyn NgynController>>;

    fn get_middlewares(&self) -> Vec<std::sync::Arc<dyn NgynMiddleware>>;
}
