use crate::server::NgynServer;

use ngyn_shared::{enums::HttpMethod, NgynModule};

/// The `NgynFactory` struct is used to create instances of `NgynServer`.
pub struct NgynFactory {}

impl NgynFactory {
    /// The `create` method takes a generic parameter `AppModule` that implements the `NgynModule` trait.
    /// It returns an instance of `NgynServer`.
    ///
    /// # Example
    ///
    /// ```
    /// use ngyn_core::{module, NgynFactory};
    ///
    /// #[module]
    /// pub struct YourAppModule;
    ///
    /// let server = NgynFactory::create::<YourAppModule>();
    /// ```
    pub fn create<AppModule: NgynModule>() -> NgynServer {
        let module = AppModule::new();
        let mut server = NgynServer::new();
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
