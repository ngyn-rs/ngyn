use crate::RustleInjectable;

use super::controller::RustleController;

/// `RustleModule` is a trait that defines the basic structure of a module in Rustle.
/// It requires two generic parameters: `C` for the controller type, and `P` for the provider type.
pub trait RustleModule {
    /// Creates a new instance of the module.
    fn new() -> Self;

    /// Returns the controllers of the module.
    /// It is expected that this method will be overridden in the implementation of the module.
    fn get_controllers(&self) -> Vec<std::sync::Arc<dyn RustleController>>;

    /// Returns the providers of the module.
    /// It is expected that this method will be overridden in the implementation of the module.
    fn get_providers(&self) -> Vec<std::sync::Arc<dyn RustleInjectable>>;
}
