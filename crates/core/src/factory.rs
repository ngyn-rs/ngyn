use ngyn_hyper::HyperApplication;
use ngyn_shared::{core::NgynEngine, traits::NgynModule};

/// The `NgynFactory` struct is used to create instances of `NgynEngine`.
pub struct NgynFactory<Application: NgynEngine = HyperApplication> {
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
    ///
    /// #[module]
    /// pub struct YourAppModule;
    ///
    /// let server: HyperApplication = NgynFactory::create::<YourAppModule>();
    /// ```
    pub fn create<AppModule: NgynModule + 'static>() -> Application {
        Application::build::<AppModule>()
    }
}
