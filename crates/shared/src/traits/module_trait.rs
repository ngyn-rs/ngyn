use crate::NgynController;

/// `NgynModule` is a trait that defines the basic structure of a module in Ngyn.
pub trait NgynModule {
    /// Creates a new instance of the module.
    /// This is for internal use only.
    fn new() -> Self;

    /// Returns the controllers of the module.
    fn get_controllers(&self) -> Vec<std::sync::Arc<dyn NgynController>>;
}
