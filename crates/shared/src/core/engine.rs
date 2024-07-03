use std::sync::{Arc, Mutex};

use hyper::Request;

use super::{Handler, RouteHandle};
use crate::{
    server::{
        context::AppState,
        response::{Middlewares, ResponseBuilder, Routes},
        Method, NgynContext, NgynResponse,
    },
    traits::{NgynMiddleware, NgynModule},
};

#[derive(Default)]
pub struct PlatformData {
    routes: Arc<Mutex<Routes>>,
    middlewares: Arc<Mutex<Middlewares>>,
    state: Option<Arc<dyn AppState>>,
}

/// Represents platform data.
impl PlatformData {
    /// Process and responds to a request asynchronously.
    ///
    /// # Arguments
    ///
    /// * `req` - The request to respond to.
    ///
    /// # Returns
    ///
    /// The response to the request.
    pub async fn respond(&self, req: Request<Vec<u8>>) -> NgynResponse {
        let state = self.state.as_ref().unwrap().clone();
        NgynResponse::build_with_state(req, self.routes.clone(), self.middlewares.clone(), state)
            .await
    }

    /// Adds a route to the platform data.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the route.
    /// * `method` - The HTTP method of the route.
    /// * `handler` - The handler function for the route.
    pub(crate) fn add_route(&mut self, path: String, method: Method, handler: Box<Handler>) {
        self.routes.lock().unwrap().push((path, method, handler));
    }

    /// Adds a middleware to the platform data.
    ///
    /// # Arguments
    ///
    /// * `middleware` - The middleware to add.
    pub(crate) fn add_middleware(&mut self, middleware: Box<dyn NgynMiddleware>) {
        self.middlewares.lock().unwrap().push(middleware);
    }
}

pub trait NgynPlatform: Default {
    fn data_mut(&mut self) -> &mut PlatformData;
}

impl<T: NgynPlatform> NgynEngine for T {}

pub trait NgynEngine: NgynPlatform {
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
        self.data_mut().add_route(path.to_string(), method, handler);
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
        self.data_mut().add_middleware(Box::new(middleware));
    }

    fn use_state(&mut self, state: impl AppState) {
        self.data_mut().state = Some(Arc::new(state));
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
