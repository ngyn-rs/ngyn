pub struct RustleFactory {}

pub trait Creatable {
    fn new() -> Self;
}

impl RustleFactory {
    pub fn create<T: Creatable>() -> tide::Server<()> {
        let app = tide::new();
        // app.with(module);
        app
    }
}
