use crate::traits::NgynController;

pub(crate) type BoxedController = Box<dyn NgynController>;

/// `NgynModule` is a trait that defines the basic structure of a module in Ngyn.
pub trait NgynModule: Send + Sync {
    /// Creates a new instance of the module.
    fn new() -> Self
    where
        Self: Sized;

    /// Returns the name of the module.
    fn name(&self) -> &str;

    /// Returns the controllers of the module.
    fn get_controllers(&self) -> Vec<BoxedController> {
        vec![]
    }
}
