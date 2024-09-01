use bytes::Bytes;
use http::Request;
use http_body_util::Full;
use std::sync::Arc;

use super::{Handler, RouteHandle};
use crate::{
    server::{context::AppState, Method, Middlewares, NgynContext, NgynResponse, Routes},
    traits::{NgynController, NgynInterpreter, NgynMiddleware, NgynModule},
};

#[derive(Default)]
pub struct PlatformData {
    routes: Routes,
    middlewares: Middlewares,
    interpreters: Vec<Box<dyn NgynInterpreter>>,
    state: Option<Arc<dyn AppState>>,
}

/// Represents platform data.
impl PlatformData {
    /// Process and responds to a request asynchronously.
    ///
    /// ### Arguments
    ///
    /// * `req` - The request to respond to.
    ///
    /// ### Returns
    ///
    /// The response to the request.
    pub async fn respond(&self, req: Request<Vec<u8>>) -> NgynResponse {
        let mut cx = NgynContext::from_request(req);
        let mut res = NgynResponse::default();

        if let Some(state) = &self.state {
            cx.set_state(state.clone());
        }

        let route_handler = self
            .routes
            .iter()
            .find_map(|(path, method, route_handler)| {
                if cx.with(path, method.as_ref()).is_some() {
                    return Some(route_handler);
                }
                None
            });

        // trigger global middlewares
        for middleware in &self.middlewares {
            middleware.handle(&mut cx, &mut res);
        }

        // execute controlled route if it is handled
        if let Some(route_handler) = route_handler {
            route_handler(&mut cx, &mut res);
            cx.execute(&mut res).await;
            // if the request method is HEAD, we should not return a body
            // even if the route handler has set a body
            if cx.request().method() == Method::HEAD {
                *res.body_mut() = Full::new(Bytes::default());
            }
        }

        // trigger interpreters
        for interpreter in &self.interpreters {
            interpreter.interpret(&mut res).await;
        }

        res
    }

    /// Adds a route to the platform data.
    ///
    /// ### Arguments
    ///
    /// * `path` - The path of the route.
    /// * `method` - The HTTP method of the route.
    /// * `handler` - The handler function for the route.
    pub(self) fn add_route(&mut self, path: String, method: Option<Method>, handler: Box<Handler>) {
        self.routes.push((path, method, handler));
    }

    /// Adds a middleware to the platform data.
    ///
    /// ### Arguments
    ///
    /// * `middleware` - The middleware to add.
    pub(self) fn add_middleware(&mut self, middleware: Box<dyn NgynMiddleware>) {
        self.middlewares.push(middleware);
    }

    /// Adds an interpreter to the platform data.
    ///
    /// ### Arguments
    ///
    /// * `interpreter` - The interpreter to add.
    pub(self) fn add_interpreter(&mut self, interpreter: Box<dyn NgynInterpreter>) {
        self.interpreters.push(interpreter);
    }
}

pub trait NgynPlatform: Default {
    fn data_mut(&mut self) -> &mut PlatformData;
}

impl<T: NgynPlatform> NgynEngine for T {}

pub trait NgynEngine: NgynPlatform {
    /// Adds a route to the application.
    ///
    /// ### Arguments
    ///
    /// * `path` - The path of the route.
    /// * `method` - The HTTP method of the route.
    /// * `handler` - The handler function for the route.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    /// use crate::{Method, NgynEngine};
    ///
    /// struct MyEngine;
    ///
    /// let mut engine = MyEngine::default();
    /// engine.route('/', Method::GET, Box::new(|_, _| {}));
    /// ```
    fn route(&mut self, path: &str, method: Method, handler: Box<Handler>) {
        self.data_mut()
            .add_route(path.to_string(), Some(method), handler);
    }

    fn any(&mut self, path: &str, handler: impl RouteHandle) {
        self.data_mut()
            .add_route(path.to_string(), None, handler.into());
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
    /// ### Arguments
    ///
    /// * `middleware` - The middleware to add.
    fn use_middleware(&mut self, middleware: impl NgynMiddleware + 'static) {
        self.data_mut().add_middleware(Box::new(middleware));
    }

    /// Adds an interpreter to the application.
    ///
    /// ### Arguments
    ///
    /// * `interpreter` - The interpreter to add.
    fn use_interpreter(&mut self, interpreter: impl NgynInterpreter + 'static) {
        self.data_mut().add_interpreter(Box::new(interpreter));
    }

    /// Sets the state of the application to any value that implements [`AppState`].
    ///
    /// ### Arguments
    ///
    /// * `state` - The state to set.
    fn set_state(&mut self, state: impl AppState + 'static) {
        self.data_mut().state = Some(Arc::new(state));
    }

    /// Loads a component which implements [`NgynModule`] into the application.
    ///
    /// ### Arguments
    ///
    /// * `module` - The module to load.
    fn load_module(&mut self, module: impl NgynModule + 'static) {
        for controller in module.get_controllers() {
            self.load_controller(controller);
        }
    }

    /// Loads a component which implements [`NgynController`] into the application.
    ///
    /// ### Arguments
    ///
    /// * `controller` - The arc'd controller to load.
    fn load_controller(&mut self, controller: Arc<Box<dyn NgynController + 'static>>) {
        for (path, http_method, handler) in controller.routes() {
            self.route(
                path.as_str(),
                Method::from_bytes(http_method.as_bytes()).unwrap_or_default(),
                Box::new({
                    let controller = controller.clone();
                    move |cx: &mut NgynContext, _res: &mut NgynResponse| {
                        let controller = controller.clone();
                        cx.prepare(controller, handler.clone());
                    }
                }),
            );
        }
    }

    /// Builds the application with the specified module.
    fn build<AppModule: NgynModule + 'static>() -> Self {
        let module = AppModule::new();
        let mut server = Self::default();
        server.load_module(module);
        server
    }
}
