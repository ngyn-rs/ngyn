use rustle_shared::RustleModule;

pub struct RustleFactory {}

impl RustleFactory {
    pub fn create<T: RustleModule>() -> tide::Server<()> {
        let app = tide::new();
        // app.with(module);
        app
    }
}
