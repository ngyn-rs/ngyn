use crate::NgynController;

/// `NgynModule` is a trait that defines the basic structure of a module in Ngyn.
pub trait NgynModule: Send + Sync {
    /// Creates a new instance of the module.
    fn new(middlewares: Vec<std::sync::Arc<dyn super::NgynMiddleware>>) -> Self
    where
        Self: Sized;

    /// Returns the name of the module.
    fn name(&self) -> &str;

    /// Returns the controllers of the module.
    fn get_controllers(&mut self) -> Vec<std::sync::Arc<dyn NgynController>>;
}
