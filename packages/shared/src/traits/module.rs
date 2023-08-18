/// `RustleModule` is a trait that defines the basic structure of a module in Rustle.
/// It requires two generic parameters: `C` for the controller type, and `P` for the provider type.
pub trait RustleModule<C, P> {
    /// Creates a new instance of the module.
    fn new() -> Self;

    /// Returns the controllers of the module.
    /// It is expected that this method will be overridden in the implementation of the module.
    fn get_controllers(&self) -> C;

    /// Returns the providers of the module.
    /// It is expected that this method will be overridden in the implementation of the module.
    fn get_providers(&self) -> P;
}
