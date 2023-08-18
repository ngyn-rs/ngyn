pub trait RustleInjectable: Send + Sync {
    fn new(&self) -> Box<dyn RustleInjectable>;
}
