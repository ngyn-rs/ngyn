pub trait RustleInjectable {
    fn new(&self) -> Box<dyn RustleInjectable>;
}
