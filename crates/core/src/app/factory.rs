use crate::server::{NgynApplication, NgynEngine};

use ngyn_shared::{enums::HttpMethod, NgynModule, NgynResponse};

/// The `NgynFactory` struct is used to create instances of `NgynEngine`.
pub struct NgynFactory<Application: NgynEngine> {
    _app: Application,
}

impl NgynFactory<NgynApplication> {
    pub fn create<AppModule: NgynModule>() -> NgynApplication {
        Self::build::<AppModule>()
    }
}

impl<Application: NgynEngine> NgynFactory<Application> {
    /// The `create` method takes a generic parameter `AppModule` that implements the `NgynModule` trait.
    /// It returns an instance of `NgynEngine`.
    ///
    /// # Example
    ///
    /// ```
    /// use ngyn::{module, NgynFactory};
    ///
    /// #[module]
    /// pub struct YourAppModule;
    ///
    /// let server = NgynFactory::create::<YourAppModule>();
    /// ```
    fn build<AppModule: NgynModule>() -> Application {
        let module = AppModule::new();
        let mut server = Application::new();
        for controller in module.get_controllers() {
            println!("Registering controller: {}", controller.name());
            for (path, http_method, handler) in controller.routes() {
                let http_method = HttpMethod::from(http_method);
                server.route(
                    path.as_str(),
                    http_method,
                    Box::new({
                        let controller = controller.clone();
                        move |req, res: NgynResponse| {
                            res.from_route(controller.clone(), handler.clone(), req)
                        }
                    }),
                );
            }
        }
        server
    }
}
