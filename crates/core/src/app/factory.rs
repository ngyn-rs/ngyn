use ngyn_shared::{core::NgynEngine, traits::NgynModule};

/// The `NgynFactory` struct is used to create instances of `NgynEngine`.
pub struct NgynFactory<Application: NgynEngine> {
    /// this is just a placeholder and would prolly not be used
    _app: Application,
}

impl<Application: NgynEngine> NgynFactory<Application> {
    /// The `create` method takes a generic parameter `AppModule` that implements the `NgynModule` trait.
    /// It returns an instance of `NgynEngine`.
    ///
    /// ### Example
    ///
    /// ```rust ignore
    /// use ngyn::prelude::*;
    /// use ngyn_hyper::HyperApplication;
    ///
    /// #[module]
    /// pub struct YourAppModule;
    ///
    /// let server = NgynFactory::<HyperApplication>::create::<YourAppModule>();
    /// ```
    pub fn create<AppModule: NgynModule>() -> Application {
        Application::build::<AppModule>()
    }
}
