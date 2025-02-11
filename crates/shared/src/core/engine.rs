use bytes::Bytes;
use http::Request;
use matchit::{Match, Router};
use std::{mem::ManuallyDrop, sync::Arc};

use super::handler::{handler, RouteHandler};
use crate::{
    server::{context::AppState, Method, NgynContext, NgynResponse, ToBytes},
    Middleware, NgynMiddleware,
};

pub struct GroupRouter<'b> {
    base_path: &'b str,
    router: Router<RouteHandler>,
}

impl RouteInstance for GroupRouter<'_> {
    fn router_mut(&mut self) -> &mut Router<RouteHandler> {
        &mut self.router
    }

    fn mount(&self) -> &str {
        self.base_path
    }
}

#[derive(Default)]
pub struct PlatformData {
    router: Router<RouteHandler>,
    middlewares: Vec<Box<dyn crate::Middleware>>,
    state: Option<Arc<Box<dyn AppState>>>,
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
        let path = req.method().to_string() + req.uri().path();
        let mut cx = NgynContext::from_request(req);

        if let Some(state) = &self.state {
            cx.state = Some(ManuallyDrop::new(state.into()));
        }

        let mut route_handler = None;
        let route_info = self.router.at(&path);

        if let Ok(Match { params, value, .. }) = route_info {
            cx.params = Some(params);
            route_handler = Some(value);
        } else {
            // if no route is found, we should return a 404 response
            *cx.response_mut().status_mut() = http::StatusCode::NOT_FOUND;
        }

        // trigger global middlewares
        for middleware in &self.middlewares {
            middleware.run(&mut cx).await;
        }

        // run the route handler
        if let Some(route_handler) = route_handler {
            *cx.response_mut().body_mut() = match route_handler {
                RouteHandler::Sync(handler) => handler(&mut cx),
                RouteHandler::Async(async_handler) => async_handler(&mut cx).await,
            }
            .to_bytes()
            .into();
            // if the request method is HEAD, we should not return a body
            // even if the route handler has set a body
            if cx.request().method() == Method::HEAD {
                *cx.response_mut().body_mut() = Bytes::default().into();
            }
        }

        cx.response
    }

    /// Adds a middleware to the platform data.
    ///
    /// ### Arguments
    ///
    /// * `middleware` - The middleware to add.
    pub(self) fn add_middleware(&mut self, middleware: Box<dyn Middleware>) {
        self.middlewares.push(middleware);
    }
}

pub trait NgynPlatform: Default {
    fn data_mut(&mut self) -> &mut PlatformData;
}

pub trait RouteInstance {
    fn router_mut(&mut self) -> &mut Router<RouteHandler>;

    /// Mounts the route on a path, defaults to "/"
    fn mount(&self) -> &str {
        "/"
    }

    /// Adds a route to the platform data.
    ///
    /// ### Arguments
    ///
    /// * `path` - The path of the route.
    /// * `method` - The HTTP method of the route.
    /// * `handler` - The handler function for the route.
    fn add_route(&mut self, path: &str, method: Option<Method>, handler: RouteHandler) {
        let method = method
            .map(|method| method.to_string())
            .unwrap_or_else(|| "{METHOD}".to_string());

        let route = if path.starts_with('/') {
            method + path
        } else {
            method + self.mount() + path
        };

        self.router_mut().insert(route, handler).unwrap();
    }
}

pub trait NgynHttpPlatform: Default {
    fn data_mut(&mut self) -> &mut PlatformData;
}

pub trait NgynHttpEngine: NgynPlatform {
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
    /// # use crate::{Method, NgynEngine};
    ///
    /// struct MyEngine;
    ///
    /// let mut engine = MyEngine::default();
    /// engine.route('/', Method::GET, Box::new(|_, _| {}));
    /// ```
    fn route(&mut self, path: &str, method: Method, handler: impl Into<RouteHandler>) {
        self.add_route(path, Some(method), handler.into());
    }

    /// Adds a new route to the `NgynApplication` with the `Method::Get`.
    fn get(&mut self, path: &str, handler: impl Into<RouteHandler>) {
        self.route(path, Method::GET, handler.into())
    }

    /// Adds a new route to the `NgynApplication` with the `Method::Post`.
    fn post(&mut self, path: &str, handler: impl Into<RouteHandler>) {
        self.route(path, Method::POST, handler.into())
    }

    /// Adds a new route to the `NgynApplication` with the `Method::Put`.
    fn put(&mut self, path: &str, handler: impl Into<RouteHandler>) {
        self.route(path, Method::PUT, handler.into())
    }

    /// Adds a new route to the `NgynApplication` with the `Method::Delete`.
    fn delete(&mut self, path: &str, handler: impl Into<RouteHandler>) {
        self.route(path, Method::DELETE, handler.into())
    }

    /// Adds a new route to the `NgynApplication` with the `Method::Patch`.
    fn patch(&mut self, path: &str, handler: impl Into<RouteHandler>) {
        self.route(path, Method::PATCH, handler.into())
    }

    /// Adds a new route to the `NgynApplication` with the `Method::Head`.
    fn head(&mut self, path: &str, handler: impl Into<RouteHandler>) {
        self.route(path, Method::HEAD, handler.into())
    }

    /// Sets up static file routes.
    ///
    /// This is great for apps tha would want to output files in a specific folder.
    /// For instance, a `public` directory can be set up and include all files in the directory
    ///
    /// The behavior of `use_static` in ngyn is different from other frameworks.
    /// 1. You can call it multiple times, each call registers a new set of routes
    /// 2. The files in `path_buf` folder aren't embedded into your binary and must be copied to the location of your binary
    ///
    /// ### Arguments
    ///
    /// - `path_buf` - static folder, relative to Cargo.toml in dev, and the binary in release
    ///
    fn use_static(&mut self, path_buf: std::path::PathBuf) -> std::io::Result<()> {
        let assets = include!("statics.rs");

        for (file_path, content) in assets {
            self.get(&file_path, handler(move |_| Bytes::from(content)));
        }

        Ok(())
    }
}

pub trait NgynEngine: NgynPlatform {
    fn any(&mut self, path: &str, handler: impl Into<RouteHandler>) {
        self.add_route(path, None, handler.into());
    }

    /// Groups related routes
    fn group(&mut self, base_path: &str, registry: impl Fn(&mut GroupRouter)) {
        let mut group = GroupRouter {
            base_path,
            router: Router::<RouteHandler>::new(),
        };
        registry(&mut group);
        self.data_mut().router.merge(group.router).unwrap();
    }

    /// Adds a middleware to the application.
    ///
    /// ### Arguments
    ///
    /// * `middleware` - The middleware to add.
    fn use_middleware(&mut self, middleware: impl NgynMiddleware + 'static) {
        self.data_mut().add_middleware(Box::new(middleware));
    }

    /// Sets the state of the application to any value that implements [`AppState`].
    ///
    /// ### Arguments
    ///
    /// * `state` - The state to set.
    fn set_state(&mut self, state: impl AppState + 'static) {
        self.data_mut().state = Some(Arc::new(Box::new(state)));
    }
}

impl<T: NgynHttpPlatform> NgynPlatform for T {
    fn data_mut(&mut self) -> &mut PlatformData {
        self.data_mut()
    }
}

impl<T: NgynPlatform> NgynEngine for T {}
impl<T: NgynPlatform> RouteInstance for T {
    fn router_mut(&mut self) -> &mut Router<RouteHandler> {
        &mut self.data_mut().router
    }
}
impl<T: NgynHttpPlatform> NgynHttpEngine for T {}

#[cfg(test)]
mod tests {
    use http::StatusCode;

    use crate::core::handler::Handler;
    use std::any::Any;

    use super::*;

    struct MockAppState;

    impl AppState for MockAppState {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    struct MockMiddleware;

    impl NgynMiddleware for MockMiddleware {
        async fn handle(cx: &mut NgynContext<'_>) {
            *cx.response_mut().status_mut() = StatusCode::OK;
        }
    }

    #[derive(Default)]
    struct MockEngine {
        data: PlatformData,
    }

    impl NgynPlatform for MockEngine {
        fn data_mut(&mut self) -> &mut PlatformData {
            &mut self.data
        }
    }

    #[tokio::test]
    async fn test_respond_with_state() {
        let mut engine = MockEngine::default();
        let app_state = MockAppState;
        engine.data_mut().state = Some(Arc::new(Box::new(app_state)));

        let req = Request::builder()
            .method(Method::GET)
            .uri("/test")
            .body(Vec::new())
            .unwrap();

        let res = engine.data.respond(req).await;

        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
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

        assert_eq!(res.status(), http::StatusCode::NOT_FOUND);
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
        let mut engine = MockEngine::default();
        let handler: Box<Handler> = Box::new(|_| Box::new(()) as Box<dyn ToBytes>);
        engine.add_route("/test", Some(Method::GET), RouteHandler::Sync(handler));

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

    // #[tokio::test]
    // async fn test_respond_with_head_method() {
    //     let mut engine = MockEngine::default();
    //     let handler: Box<Handler> = Box::new(|_| {});
    //     engine
    //         .data_mut()
    //         .add_route("/test", Some(Method::GET), RouteHandler::Sync(handler));

    //     let req = Request::builder()
    //         .method(Method::GET)
    //         .uri("/test")
    //         .body(Vec::new())
    //         .unwrap();

    //     let res = engine.data.respond(req).await;

    //     assert_eq!(res.status(), http::StatusCode::OK);
    // }

    #[tokio::test]
    async fn test_add_route() {
        let mut engine = MockEngine::default();
        let handler: Box<Handler> = Box::new(|_| Box::new(()) as Box<dyn ToBytes>);
        engine.add_route("/test", Some(Method::GET), RouteHandler::Sync(handler));

        assert!(engine.data.router.at("GET/test").is_ok());
    }

    #[tokio::test]
    async fn test_add_middleware() {
        let mut engine = MockEngine::default();
        let middleware = MockMiddleware;
        engine.data_mut().add_middleware(Box::new(middleware));

        assert_eq!(engine.data.middlewares.len(), 1);
    }

    #[tokio::test]
    async fn test_use_middleware() {
        let mut engine = MockEngine::default();
        let middleware = MockMiddleware;
        engine.use_middleware(middleware);

        assert_eq!(engine.data.middlewares.len(), 1);
    }

    #[tokio::test]
    async fn test_set_state() {
        let mut engine = MockEngine::default();
        let app_state = MockAppState;
        engine.set_state(app_state);

        assert!(engine.data.state.is_some());
    }
}
