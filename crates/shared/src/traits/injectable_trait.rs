/// `NgynInjectableInit` is a trait that handles the creation of new injectables.
/// It is designed to be thread-safe.
pub trait NgynInjectableInit: Send + Sync {
    /// Creates a new instance of the injectable.
    /// This is for internal use only.
    fn new() -> Self;

    /// Returns a boxed `NgynInjectable`.
    /// This is for internal use only.
    fn boxed() -> Box<dyn NgynInjectable>;
}

/// `NgynInjectable` is a trait that defines the basic structure of an injectable in Ngyn.
/// It is designed to be thread-safe.
pub trait NgynInjectable: Send + Sync {
    /// Returns the name of the injectable.
    fn name(&self) -> &str;
}
