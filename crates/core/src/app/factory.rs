use crate::server::RustleServer;

use rustle_shared::{enums::HttpMethod, RustleModule};

/// The `RustleFactory` struct is used to create instances of `RustleServer`.
pub struct RustleFactory {}

impl RustleFactory {
    /// The `create` method takes a generic parameter `AppModule` that implements the `RustleModule` trait.
    /// It returns an instance of `RustleServer`.
    ///
    /// # Example
    ///
    /// ```
    /// use rustle_core::{module, RustleFactory};
    /// 
    /// #[module]
    /// pub struct YourAppModule;
    /// 
    /// let server = RustleFactory::create::<YourAppModule>();
    /// ```
    pub fn create<AppModule: RustleModule>() -> RustleServer {
        let module = AppModule::new();
        let mut server = RustleServer::new();
        for controller in module.get_controllers() {
            println!("Registering controller: {}", controller.name());
            for (path, http_method, handler) in controller.routes() {
                let http_method = HttpMethod::from_str(http_method.as_str()).unwrap();
                server.route(
                    path.as_str(),
                    http_method,
                    Box::new({
                        let controller = controller.clone();
                        move |req, res| controller.handle(handler.clone(), req, res)
                    }),
                );
            }
        }
        server
    }
}
