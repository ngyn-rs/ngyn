use std::sync::Arc;

use ngyn_shared::{Method, NgynContext, NgynEngine, NgynModule, NgynResponse};

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
    /// use ngyn::{platforms::HyperApplication, prelude::*};
    ///
    /// #[module]
    /// pub struct YourAppModule;
    ///
    /// let server = NgynFactory::<HyperApplication>::create::<YourAppModule>();
    /// ```
    pub fn create<AppModule: NgynModule>() -> Application {
        let mut module = AppModule::new(vec![]);
        let mut server = Application::default();
        for controller in module.get_controllers() {
            for (path, http_method, handler) in controller.routes() {
                server.route(
                    path.as_str(),
                    Method::from_bytes(http_method.to_uppercase().as_bytes()).unwrap(),
                    Box::new({
                        let controller = controller.clone();
                        move |cx: &mut NgynContext, _res: &mut NgynResponse| {
                            cx.prepare(Arc::clone(&controller), handler.clone());
                        }
                    }),
                );
            }
        }
        server
    }
}
