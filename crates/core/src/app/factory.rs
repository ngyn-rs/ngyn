#[cfg(feature = "tide")]
use crate::platforms::NgynApplication;

use ngyn_shared::{enums::HttpMethod, NgynEngine, NgynModule, NgynRequest, NgynResponse};

/// The `NgynFactory` struct is used to create instances of `NgynEngine`.
pub struct NgynFactory<Application: NgynEngine> {
    /// this is just a placeholder and would prolly not be used
    _app: Application,
}

#[cfg(feature = "tide")]
impl NgynFactory<NgynApplication> {
    pub fn create<AppModule: NgynModule>() -> NgynApplication {
        Self::build::<AppModule>()
    }
}

#[cfg(feature = "vercel")]
impl NgynFactory<crate::platforms::VercelApplication> {
    pub fn create<AppModule: NgynModule>() -> crate::platforms::VercelApplication {
        Self::build::<AppModule>()
    }
}

impl<Application: NgynEngine> NgynFactory<Application> {
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
    fn build<AppModule: NgynModule>() -> Application {
        let mut module = AppModule::new(vec![]);
        let mut server = Application::new();
        for controller in module.get_controllers() {
            println!("Registering controller: {}", controller.name());
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
