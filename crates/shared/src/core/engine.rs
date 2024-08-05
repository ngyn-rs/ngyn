use http_body_util::Full;
use hyper::{body::Bytes, Request, Response};
use std::sync::Arc;

use super::{Handler, RouteHandle};
use crate::{
    server::{
        context::AppState,
        response::{Middlewares, Routes},
        Method, NgynContext, NgynResponse,
    },
    traits::{NgynController, NgynMiddleware, NgynModule},
};

#[derive(Default)]
pub struct PlatformData {
    routes: Routes,
    middlewares: Middlewares,
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
        let mut cx = NgynContext::from_request(req);
        let mut res = Response::new(Full::new(Bytes::default()));

        if let Some(state) = &self.state {
            cx.set_state(state.clone());
        }

        let mut is_handled = false;

        let _ = self
            .routes
            .iter()
            .for_each(|(path, method, route_handler)| {
                if !is_handled && cx.with(path, method).is_some() {
                    is_handled = true;
                    // trigger global middlewares
                    self.middlewares
                        .iter()
                        .for_each(|middlewares| middlewares.handle(&mut cx, &mut res));
                    // trigger route handler
                    route_handler(&mut cx, &mut res);
                }
            });

        // execute controlled route if it is handled
        if is_handled {
            cx.execute(&mut res).await;
        } else {
            // trigger global middlewares if no route is found
            self.middlewares
                .iter()
                .for_each(|middlewares| middlewares.handle(&mut cx, &mut res));
        }

        res
    }

    /// Adds a route to the platform data.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the route.
    /// * `method` - The HTTP method of the route.
    /// * `handler` - The handler function for the route.
    pub(crate) fn add_route(&mut self, path: String, method: Method, handler: Box<Handler>) {
        self.routes.push((path, method, handler));
    }

    /// Adds a middleware to the platform data.
    ///
    /// # Arguments
    ///
    /// * `middleware` - The middleware to add.
    pub(crate) fn add_middleware(&mut self, middleware: Box<dyn NgynMiddleware>) {
        self.middlewares.push(middleware);
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
    /// engine.route('/', Method::GET, Box::new(|_, _| {}));
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

    fn set_state(&mut self, state: impl AppState + 'static) {
        self.data_mut().state = Some(Arc::new(state));
    }

    fn load_module(&mut self, module: impl NgynModule + 'static) {
        for controller in module.get_controllers() {
            self.load_controller(controller);
        }
    }

    fn load_controller(&mut self, controller: Arc<Box<dyn NgynController + 'static>>) {
        for (path, http_method, handler) in controller.routes() {
            self.route(
                path.as_str(),
                hyper::Method::from_bytes(http_method.as_bytes()).unwrap_or_default(),
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

    fn build<AppModule: NgynModule + 'static>() -> Self {
        let module = AppModule::new();
        let mut server = Self::default();
        server.load_module(module);
        server
    }
}
