use crate::server::RustleServer;

use rustle_shared::{enums::HttpMethod, RustleModule};

/// The `RustleFactory` struct is used to create instances of `RustleServer`.
pub struct RustleFactory {}

impl RustleFactory {
    /// The `create` function takes a generic parameter `AppModule` that implements the `RustleModule` trait.
    /// It returns an instance of `RustleServer`.
    ///
    /// # Example
    ///
    /// ```
    /// let server = RustleFactory::create::<YourAppModule>();
    /// ```
    pub fn create<AppModule: RustleModule>() -> RustleServer {
        let module = AppModule::new();
        let mut server = RustleServer::new();
        module.get_controllers().iter().for_each(|controller| {
            println!("Registering controller: {}", controller.name());
            // print count of routes
            println!("Routes: {}", controller.routes().len());
            for (path, http_method, handler) in controller.routes() {
                let http_method = HttpMethod::from_str(http_method.as_str()).unwrap();
                server.route(path.as_str(), http_method, handler);
            }
        });
        server
    }
}
