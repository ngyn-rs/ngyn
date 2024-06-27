use super::{Handler, RouteHandle};
use crate::{
    server::{Method, NgynContext, NgynResponse},
    traits::{NgynMiddleware, NgynModule},
};

#[derive(Default)]
pub struct PlatformData {
    pub routes: Vec<(String, Method, Box<Handler>)>,
    pub middlewares: Vec<Box<dyn NgynMiddleware>>,
}

pub trait NgynEngine: Default {
    fn data_mut(&mut self) -> &mut PlatformData;

    /// Adds a route to the application.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the route.
    /// * `method` - The HTTP method of the route.
    /// * `handler` - The handler function for the route.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use crate::{Method, NgynEngine};
    ///
    /// struct MyEngine;
    ///
    /// let mut engine = MyEngine::default();
    /// engine.route("/", Method::GET, Box::new(|_, _| {}));
    /// ```
    fn route(&mut self, path: &str, method: Method, handler: Box<Handler>) {
        self.data_mut()
            .routes
            .push((path.to_string(), method, handler));
    }

    /// Adds a new route to the `NgynApplication` with the `Method::Get`.
    fn get(&mut self, path: &str, handler: impl RouteHandle) {
        self.route(path, Method::GET, handler.into())
    }

    /// Adds a new route to the `NgynApplication` with the `Method::Post`.
    fn post(&mut self, path: &str, handler: impl RouteHandle) {
        self.route(path, Method::POST, handler.into())
    }

    /// Adds a new route to the `NgynApplication` with the `Method::Put`.
    fn put(&mut self, path: &str, handler: impl RouteHandle) {
        self.route(path, Method::PUT, handler.into())
    }

    /// Adds a new route to the `NgynApplication` with the `Method::Delete`.
    fn delete(&mut self, path: &str, handler: impl RouteHandle) {
        self.route(path, Method::DELETE, handler.into())
    }

    /// Adds a new route to the `NgynApplication` with the `Method::Patch`.
    fn patch(&mut self, path: &str, handler: impl RouteHandle) {
        self.route(path, Method::PATCH, handler.into())
    }

    /// Adds a new route to the `NgynApplication` with the `Method::Head`.
    fn head(&mut self, path: &str, handler: impl RouteHandle) {
        self.route(path, Method::HEAD, handler.into())
    }

    /// Adds a middleware to the application.
    ///
    /// # Arguments
    ///
    /// * `middleware` - The middleware to add.
    fn use_middleware(&mut self, middleware: impl NgynMiddleware + 'static) {
        self.data_mut()
            .middlewares
            .push(Box::new(middleware) as Box<dyn NgynMiddleware>);
    }

    fn build<AppModule: NgynModule>() -> Self {
        let mut module = AppModule::new();
        let mut server = Self::default();
        for controller in module.get_controllers() {
            for (path, http_method, handler) in controller.routes() {
                server.route(
                    path.as_str(),
                    Method::from_bytes(http_method.to_uppercase().as_bytes()).unwrap(),
                    Box::new({
                        let controller = controller.clone();
                        move |cx: &mut NgynContext, _res: &mut NgynResponse| {
                            cx.prepare(controller.clone(), handler.clone());
                        }
                    }),
                );
            }
        }
        server
    }
}
