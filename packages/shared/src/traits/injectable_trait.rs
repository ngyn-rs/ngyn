/// `RustleInjectable` is a trait that defines the basic structure of an injectable in Rustle.
/// It is designed to be thread-safe.
pub trait RustleInjectable: Send + Sync {
    /// Creates a new instance of the injectable.
    /// This is for internal use only.
    fn new(&self) -> Box<dyn RustleInjectable + Send + Sync>;
}
