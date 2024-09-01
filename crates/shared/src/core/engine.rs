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
                if cx.with(path, method).is_some() {
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
    pub(crate) fn add_route(&mut self, path: String, method: Method, handler: Box<Handler>) {
        self.routes.push((path, method, handler));
    }

    /// Adds a middleware to the platform data.
    ///
    /// ### Arguments
    ///
    /// * `middleware` - The middleware to add.
    pub(crate) fn add_middleware(&mut self, middleware: Box<dyn NgynMiddleware>) {
        self.middlewares.push(middleware);
    }

    /// Adds an interpreter to the platform data.
    ///
    /// ### Arguments
    ///
    /// * `interpreter` - The interpreter to add.
    pub(crate) fn add_interpreter(&mut self, interpreter: Box<dyn NgynInterpreter>) {
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
                http::Method::from_bytes(http_method.as_bytes()).unwrap_or_default(),
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

#[cfg(test)]
mod tests {
    use crate::traits::NgynInjectable;

    use super::*;

    struct MockAppState;

    impl AppState for MockAppState {}

    struct MockMiddleware;

    impl NgynInjectable for MockMiddleware {
        fn new() -> Self
        where
            Self: Sized,
        {
            Self {}
        }
    }

    impl NgynMiddleware for MockMiddleware {
        fn handle(&self, _cx: &mut NgynContext, _res: &mut NgynResponse) {}
    }

    struct MockInterpreter;

    #[async_trait::async_trait]
    impl NgynInterpreter for MockInterpreter {
        async fn interpret(&self, _res: &mut NgynResponse) {}
    }

    struct MockController;

    impl NgynInjectable for MockController {
        fn new() -> Self
        where
            Self: Sized,
        {
            Self {}
        }
    }

    impl NgynController for MockController {
        fn routes(&self) -> Vec<(String, String, String)> {
            vec![(
                "/test".to_string(),
                Method::GET.to_string(),
                "handler".to_string(),
            )]
        }
    }

    struct MockModule;

    impl NgynModule for MockModule {
        fn new() -> Self {
            Self {}
        }

        fn get_controllers(&self) -> Vec<Arc<Box<dyn NgynController>>> {
            vec![Arc::new(Box::new(MockController) as Box<dyn NgynController>)]
        }
    }

    struct MockEngine {
        data: PlatformData,
    }

    impl NgynPlatform for MockEngine {
        fn data_mut(&mut self) -> &mut PlatformData {
            &mut self.data
        }
    }

    impl Default for MockEngine {
        fn default() -> Self {
            Self {
                data: PlatformData::default(),
            }
        }
    }

    #[tokio::test]
    async fn test_respond_with_state() {
        let mut engine = MockEngine::default();
        let app_state = MockAppState;
        engine.data_mut().state = Some(Arc::new(app_state));

        let req = Request::builder()
            .method(Method::GET)
            .uri("/test")
            .body(Vec::new())
            .unwrap();

        let res = engine.data.respond(req).await;

        assert_eq!(res.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_respond_without_state() {
        let engine = MockEngine::default();

        let req = Request::builder()
            .method(Method::GET)
            .uri("/test")
            .body(Vec::new())
            .unwrap();

        let res = engine.data.respond(req).await;

        assert_eq!(res.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_respond_with_middleware() {
        let mut engine = MockEngine::default();
        let middleware = MockMiddleware;
        engine.data_mut().add_middleware(Box::new(middleware));

        let req = Request::builder()
            .method(Method::GET)
            .uri("/test")
            .body(Vec::new())
            .unwrap();

        let res = engine.data.respond(req).await;

        assert_eq!(res.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_respond_with_route_handler() {
        let engine = MockEngine::default();
        let handler: Box<Handler> = Box::new(|_, _| {});
        engine
            .data_mut()
            .add_route("/test".to_string(), Some(Method::GET), handler);

        let req = Request::builder()
            .method(Method::GET)
            .uri("/test")
            .body(Vec::new())
            .unwrap();

        let res = engine.data.respond(req).await;

        assert_eq!(res.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_respond_with_route_handler_not_found() {
        let engine = MockEngine::default();

        let req = Request::builder()
            .method(Method::GET)
            .uri("/test")
            .body(Vec::new())
            .unwrap();

        let res = engine.data.respond(req).await;

        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_respond_with_interpreter() {
        let mut engine = MockEngine::default();
        let interpreter = MockInterpreter;
        engine.data_mut().add_interpreter(Box::new(interpreter));

        let req = Request::builder()
            .method(Method::GET)
            .uri("/test")
            .body(Vec::new())
            .unwrap();

        let res = engine.data.respond(req).await;

        assert_eq!(res.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_respond_with_head_method() {
        let mut engine = MockEngine::default();
        let handler: Box<Handler> = Box::new(|_, _| {});
        engine
            .data_mut()
            .add_route("/test".to_string(), Some(Method::GET), handler);

        let req = Request::builder()
            .method(Method::HEAD)
            .uri("/test")
            .body(Vec::new())
            .unwrap();

        let res = engine.data.respond(req).await;

        assert_eq!(res.status(), http::StatusCode::OK);
        assert_eq!(res.body(), Full::new(Bytes::default()));
    }

    #[tokio::test]
    async fn test_add_route() {
        let mut engine = MockEngine::default();
        let handler: Box<Handler> = Box::new(|_, _| {});
        engine
            .data_mut()
            .add_route("/test".to_string(), Some(Method::GET), handler);

        assert_eq!(engine.data.routes.len(), 1);
    }

    #[tokio::test]
    async fn test_add_middleware() {
        let mut engine = MockEngine::default();
        let middleware = MockMiddleware;
        engine.data_mut().add_middleware(Box::new(middleware));

        assert_eq!(engine.data.middlewares.len(), 1);
    }

    #[tokio::test]
    async fn test_add_interpreter() {
        let mut engine = MockEngine::default();
        let interpreter = MockInterpreter;
        engine.data_mut().add_interpreter(Box::new(interpreter));

        assert_eq!(engine.data.interpreters.len(), 1);
    }

    #[tokio::test]
    async fn test_route() {
        let mut engine = MockEngine::default();
        let handler: Box<Handler> = Box::new(|_, _| {});
        engine.route("/test", Method::GET, handler);

        assert_eq!(engine.data.routes.len(), 1);
    }

    #[tokio::test]
    async fn test_any() {
        let mut engine = MockEngine::default();
        let handler: Box<Handler> = Box::new(|_, _| {});
        engine.any("/test", handler);

        assert_eq!(engine.data.routes.len(), 1);
    }

    #[tokio::test]
    async fn test_get() {
        let mut engine = MockEngine::default();
        let handler: Box<Handler> = Box::new(|_, _| {});
        engine.get("/test", handler);

        assert_eq!(engine.data.routes.len(), 1);
    }

    #[tokio::test]
    async fn test_post() {
        let mut engine = MockEngine::default();
        let handler: Box<Handler> = Box::new(|_, _| {});
        engine.post("/test", handler);

        assert_eq!(engine.data.routes.len(), 1);
    }

    #[tokio::test]
    async fn test_put() {
        let mut engine = MockEngine::default();
        let handler: Box<Handler> = Box::new(|_, _| {});
        engine.put("/test", handler);

        assert_eq!(engine.data.routes.len(), 1);
    }

    #[tokio::test]
    async fn test_delete() {
        let mut engine = MockEngine::default();
        let handler: Box<Handler> = Box::new(|_, _| {});
        engine.delete("/test", handler);

        assert_eq!(engine.data.routes.len(), 1);
    }

    #[tokio::test]
    async fn test_patch() {
        let mut engine = MockEngine::default();
        let handler: Box<Handler> = Box::new(|_, _| {});
        engine.patch("/test", handler);

        assert_eq!(engine.data.routes.len(), 1);
    }

    #[tokio::test]
    async fn test_head() {
        let mut engine = MockEngine::default();
        let handler: Box<Handler> = Box::new(|_, _| {});
        engine.head("/test", handler);

        assert_eq!(engine.data.routes.len(), 1);
    }

    #[tokio::test]
    async fn test_use_middleware() {
        let mut engine = MockEngine::default();
        let middleware = MockMiddleware;
        engine.use_middleware(middleware);

        assert_eq!(engine.data.middlewares.len(), 1);
    }

    #[tokio::test]
    async fn test_use_interpreter() {
        let mut engine = MockEngine::default();
        let interpreter = MockInterpreter;
        engine.use_interpreter(interpreter);

        assert_eq!(engine.data.interpreters.len(), 1);
    }

    #[tokio::test]
    async fn test_set_state() {
        let mut engine = MockEngine::default();
        let app_state = MockAppState;
        engine.set_state(app_state);

        assert!(engine.data.state.is_some());
    }

    #[tokio::test]
    async fn test_load_module() {
        let mut engine = MockEngine::default();
        let module = MockModule::new();
        engine.load_module(module);

        assert_eq!(engine.data.routes.len(), 1);
    }

    #[tokio::test]
    async fn test_load_controller() {
        let mut engine = MockEngine::default();
        let controller = Arc::new(Box::new(MockController) as Box<dyn NgynController>);
        engine.load_controller(controller);

        assert_eq!(engine.data.routes.len(), 1);
    }

    #[tokio::test]
    async fn test_build() {
        let engine: MockEngine = MockEngine::build::<MockModule>();

        assert_eq!(engine.data.routes.len(), 1);
    }
}
