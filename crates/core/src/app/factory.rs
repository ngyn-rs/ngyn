use ngyn_shared::{enums::HttpMethod, NgynEngine, NgynModule, NgynRequest, NgynResponse};

/// The `NgynFactory` struct is used to create instances of `NgynEngine`.
pub struct NgynFactory<Application: NgynEngine> {
    /// this is just a placeholder and would prolly not be used
    _app: Application,
}

impl<Application: NgynEngine> NgynFactory<Application> {
    #[allow(dead_code)]
    /// The `create` method takes a generic parameter `AppModule` that implements the `NgynModule` trait.
    /// It returns an instance of `NgynEngine`.
    ///
    /// ### Example
    ///
    /// ```
    /// use ngyn::{module, NgynFactory, platforms::NgynApplication};
    ///
    /// #[module]
    /// pub struct YourAppModule;
    ///
    /// let server = NgynFactory::<NgynApplication>::create::<YourAppModule>();
    /// ```
    pub fn create<AppModule: NgynModule>() -> Application {
        let mut module = AppModule::new(vec![]);
        let mut server = Application::new();
        for controller in module.get_controllers() {
            for (path, http_method, handler) in controller.get_routes() {
                let http_method = HttpMethod::from(http_method);
                server.route(
                    path.as_str(),
                    http_method,
                    Box::new({
                        let controller = controller.clone();
                        move |req: &NgynRequest, res: &mut NgynResponse| {
                            res.with_controller(controller.clone(), handler.clone(), req);
                        }
                    }),
                );
            }
        }
        server
    }
}
