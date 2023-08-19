pub trait RustleController: Send + Sync {
    fn new(&self) -> Box<dyn RustleController + Send + Sync>;
}
