//! Core engine implementation for the Ngyn web framework.
//!
//! This module provides the core routing and request handling functionality.
//! It implements the main routing logic, middleware processing, and error handling.

use std::{mem::ManuallyDrop, sync::Arc};

use bytes::Bytes;
use http::Request;
use matchit::{Match, Router};

use super::{
    error::NgynError,
    handler::{handler, RouteHandler},
};
use crate::{
    server::{context::AppState, Method, NgynContext, NgynResponse, ToBytes},
    Middleware, NgynMiddleware,
};

/// A router that can be used to group routes under a common path prefix.
///
/// This allows for organizing routes into logical groups and applying
/// shared middleware or path prefixes to multiple routes at once.
#[derive(Default)]
pub struct GroupRouter {
    /// The underlying platform data containing routes and middleware.
    data: PlatformData,
}

impl NgynPlatform for GroupRouter {
    fn data_mut(&mut self) -> &mut PlatformData {
        &mut self.data
    }
}

/// Type alias for error handler function
pub type ErrorHandler = Arc<dyn Fn(NgynError) + Send + Sync>;

/// Core platform data structure containing routing and middleware configuration.
///
/// This structure maintains the state of the web application, including:
/// - Registered routes and their handlers
/// - Global middleware stack
/// - Application state
/// - Error handling configuration
#[derive(Default)]
pub struct PlatformData {
    /// Base path prefix for all routes in this platform
    base_path: &'static str,
    /// Router containing all registered routes and their handlers
    router: Router<RouteHandler>,
    /// Global middleware stack that runs before route handlers.
    middlewares: Vec<Box<dyn crate::Middleware>>,
    /// Optional application state shared across all handlers
    state: Option<Arc<Box<dyn AppState>>>,
    /// Optional custom error handler
    error_handler: Option<ErrorHandler>,
}

/// Represents platform data.
impl PlatformData {
    /// Processes and responds to an HTTP request asynchronously.
    ///
    /// This method handles the complete request lifecycle:
    /// 1. Extracts path and method from the request
    /// 2. Matches the route and extracts parameters
    /// 3. Executes global middleware stack
    /// 4. Runs the matched route handler
    /// 5. Handles any errors that occur during processing
    ///
    /// # Arguments
    ///
    /// * `req` - The incoming HTTP request to process
    ///
    /// # Returns
    ///
    /// Returns a `NgynResponse` containing the response to send back to the client
    #[must_use]
    pub async fn respond(&self, req: Request<Vec<u8>>) -> NgynResponse {
        let request_path = req.uri().path().to_string();
        let path = req.method().to_string() + &request_path;
        let mut cx = NgynContext::from_request(req);

        // Initialize context with application state if available
        if let Some(state) = &self.state {
            cx.state = Some(ManuallyDrop::new(state.into()));
        }

        // Attempt to match the route and handle potential errors
        let route_handler = match self.router.at(&path) {
            Ok(Match { params, value, .. }) => {
                cx.params = Some(params);
                Some(value)
            }
            Err(e) => {
                if let Some(handler) = &self.error_handler {
                    handler(NgynError::Route(format!(
                        "No route found for {}: {}",
                        path, e
                    )));
                }
                *cx.response_mut().status_mut() = http::StatusCode::NOT_FOUND;
                None
            }
        };

        // Execute middleware stack
        for middleware in &self.middlewares {
            middleware.run(&mut cx).await;
        }

        // Execute route handler if found
        if let Some(route_handler) = route_handler {
            let response = match route_handler {
                RouteHandler::Sync(handler) => {
                    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        handler(&mut cx)
                    })) {
                        Ok(result) => result,
                        Err(_) => {
                            if let Some(handler) = &self.error_handler {
                                handler(NgynError::Handler(format!(
                                    "Handler at /{} panicked while executing",
                                    request_path
                                )));
                            }
                            *cx.response_mut().status_mut() =
                                http::StatusCode::INTERNAL_SERVER_ERROR;
                            Box::new(())
                        }
                    }
                }
                RouteHandler::Async(async_handler) => async_handler(&mut cx).await,
            };

            *cx.response_mut().body_mut() = response.to_bytes().into();

            // if the request method is HEAD, we should not return a body
            if cx.request().method() == Method::HEAD {
                *cx.response_mut().body_mut() = Bytes::default().into();
            }
        }

        cx.response
    }

    /// Adds a middleware to the global middleware stack.
    ///
    /// Middleware are executed in the order they are added, before any route handlers.
    ///
    /// # Arguments
    ///
    /// * `middleware` - The middleware implementation to add to the stack
    pub(self) fn add_middleware(&mut self, middleware: Box<dyn Middleware>) {
        self.middlewares.push(middleware);
    }

    /// Returns a reference to the current error handler, if one is set.
    ///
    /// # Returns
    ///
    /// Returns an `Option` containing a reference to the error handler function.
    pub fn error_handler(&self) -> &Option<ErrorHandler> {
        &self.error_handler
    }
}

/// Core trait that must be implemented by all Ngyn platform types.
///
/// This trait provides access to the underlying platform data and implements
/// common functionality like error handling that all platforms share.
///
/// # Safety
///
/// Implementations must ensure thread-safety when modifying platform data,
/// as it may be accessed from multiple threads simultaneously.
pub trait NgynPlatform: Default {
    /// Gets mutable access to the platform's underlying data.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the platform's [`PlatformData`].
    fn data_mut(&mut self) -> &mut PlatformData;

    /// Set a custom error handler for the platform to handle I/O errors that occur during request processing.
    ///
    /// This method allows you to define custom error handling logic that will be called whenever
    /// an I/O error occurs in the platform. The handler receives the error and can perform custom
    /// error logging, reporting, or recovery actions.
    ///
    /// ### Arguments
    ///
    /// * `handler` - A function that takes a `std::io::Error` and handles it. The handler must be
    ///   `Send + Sync + 'static` to ensure it can be safely shared across threads.
    ///
    /// ### Examples
    ///
    /// ```
    /// use ngyn_hyper::HyperApplication;
    ///
    /// let mut app = HyperApplication::default();
    ///
    /// // Add a custom error handler that logs the error
    /// app.on_error(|err| {
    ///     eprintln!("Server error occurred: {}", err);
    /// });
    /// ```
    fn on_error<F>(&mut self, handler: F)
    where
        F: Fn(NgynError) + Send + Sync + 'static,
    {
        self.data_mut().error_handler = Some(Arc::new(handler));
    }
}

/// Trait for types that can have routes registered on them.
///
/// This trait provides the core routing functionality used by both the main application
/// and route groups. It allows for registering handlers for different HTTP methods and paths.
pub trait RouteInstance: NgynPlatform {
    /// Gets mutable access to the underlying router.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the [`Router`] containing route handlers.
    fn router_mut(&mut self) -> &mut Router<RouteHandler>;

    /// Returns the base path where this route instance is mounted.
    ///
    /// # Returns
    ///
    /// Returns the mount path as a string slice, defaults to "/".
    fn mount(&mut self) -> &str {
        "/"
    }

    /// Registers a new route handler for the given path and HTTP method.
    ///
    /// This method handles several routing concerns:
    /// - Automatically adds HEAD handlers for GET routes
    /// - Handles both absolute and relative paths
    /// - Supports method-agnostic routes with {METHOD} placeholder
    ///
    /// # Arguments
    ///
    /// * `path` - The URL path pattern to match
    /// * `http_method` - Optional HTTP method to handle, or None for method-agnostic routes
    /// * `handler` - The handler function to execute when the route matches
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ngyn::Method;
    ///
    /// app.add_route("/users", Some(Method::GET), handler);
    /// app.add_route("/api/{param}", None, handler); // Matches any method
    /// ```
    fn add_route(&mut self, path: &str, http_method: Option<Method>, handler: RouteHandler) {
        // For GET routes, automatically add a HEAD handler that returns empty response
        if http_method == Some(Method::GET) {
            let head_path = format!("HEAD{}", path);
            let head_handler = RouteHandler::Sync(Box::new(|_| Box::new(Bytes::default())));
            if let Err(e) = self.router_mut().insert(head_path.clone(), head_handler) {
                if let Some(handler) = &self.data_mut().error_handler {
                    handler(NgynError::Route(format!(
                        "Failed to add HEAD route '{}': {}",
                        head_path, e
                    )));
                }
                return;
            }
        }

        // Construct the full route path
        let method_str = http_method
            .map(|method| method.to_string())
            .unwrap_or_else(|| "{METHOD}".to_string());

        let route_path = if path.starts_with('/') {
            format!("{}{}", method_str, path)
        } else {
            format!("{}{}{}", method_str, self.mount(), path)
        };

        // Register the route handler
        // Add the route to router with error handling
        if let Err(e) = self.router_mut().insert(route_path.clone(), handler) {
            if let Some(handler) = &self.data_mut().error_handler {
                handler(NgynError::Route(format!(
                    "Failed to add route '{}': {}",
                    route_path, e
                )));
            }
        }
    }
}

pub trait NgynHttpPlatform: Default {
    fn data_mut(&mut self) -> &mut PlatformData;
}

/// A trait for HTTP-specific routing functionality in the Ngyn framework.
///
/// This trait extends `NgynPlatform` to provide HTTP-specific routing methods
/// for handling different HTTP methods (GET, POST, PUT, etc.).
pub trait NgynHttpEngine: NgynPlatform {
    /// Registers a route handler for a specific HTTP method.
    ///
    /// # Arguments
    /// * `path` - The URL path pattern to match
    /// * `method` - The HTTP method to handle
    /// * `handler` - The handler function to execute
    ///
    /// # Returns
    /// Returns a `RouteResult<()>` indicating success or failure
    ///
    /// # Example
    /// ```rust
    /// use ngyn::http::Method;
    ///
    /// app.route("/api/users", Method::GET, |_req| async {
    ///     Ok(Response::new().with_body("Users list"))
    /// });
    /// ```
    fn route(&mut self, path: &str, method: Method, handler: impl Into<RouteHandler>) {
        self.add_route(path, Some(method), handler.into());
    }

    /// Adds a new route to the `NgynApplication` with the `Method::Get`.
    /// Registers a GET route handler.
    ///
    /// # Arguments
    /// * `path` - The URL path pattern to match
    /// * `handler` - The handler function to execute
    ///
    /// # Returns
    /// Returns a `RouteResult<()>` indicating success or failure
    fn get(&mut self, path: &str, handler: impl Into<RouteHandler>) {
        self.route(path, Method::GET, handler.into())
    }

    /// Registers a POST route handler.
    ///
    /// # Arguments
    /// * `path` - The URL path pattern to match
    /// * `handler` - The handler function to execute
    ///
    /// # Returns
    /// Returns a `RouteResult<()>` indicating success or failure
    fn post(&mut self, path: &str, handler: impl Into<RouteHandler>) {
        self.route(path, Method::POST, handler.into())
    }

    /// Registers a PUT route handler.
    ///
    /// # Arguments
    /// * `path` - The URL path pattern to match
    /// * `handler` - The handler function to execute
    ///
    /// # Returns
    /// Returns a `RouteResult<()>` indicating success or failure
    fn put(&mut self, path: &str, handler: impl Into<RouteHandler>) {
        self.route(path, Method::PUT, handler.into())
    }

    /// Registers a DELETE route handler.
    ///
    /// # Arguments
    /// * `path` - The URL path pattern to match
    /// * `handler` - The handler function to execute
    ///
    /// # Returns
    /// Returns a `RouteResult<()>` indicating success or failure
    fn delete(&mut self, path: &str, handler: impl Into<RouteHandler>) {
        self.route(path, Method::DELETE, handler.into())
    }

    /// Registers a PATCH route handler.
    ///
    /// # Arguments
    /// * `path` - The URL path pattern to match
    /// * `handler` - The handler function to execute
    ///
    /// # Returns
    /// Returns a `RouteResult<()>` indicating success or failure
    fn patch(&mut self, path: &str, handler: impl Into<RouteHandler>) {
        self.route(path, Method::PATCH, handler.into())
    }

    /// Registers a HEAD route handler.
    ///
    /// # Arguments
    /// * `path` - The URL path pattern to match
    /// * `handler` - The handler function to execute
    ///
    /// # Returns
    /// Returns a `RouteResult<()>` indicating success or failure
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

/// The main engine trait that combines HTTP routing capabilities with default initialization.
///
/// This trait extends `NgynHttpEngine` and requires `Default` implementation to provide
/// a complete web application engine.
pub trait NgynEngine: NgynPlatform + Default {
    /// Registers a route handler for any HTTP method.
    ///
    /// # Arguments
    /// * `path` - The URL path pattern to match
    /// * `handler` - The handler function to execute
    ///
    /// # Returns
    /// Returns a `RouteResult<()>` indicating success or failure
    fn any(&mut self, path: &str, handler: impl Into<RouteHandler>) {
        self.add_route(path, None, handler.into())
    }

    /// Groups related routes under a common base path.
    ///
    /// # Arguments
    /// * `base_path` - The common prefix for all routes in the group
    /// * `registry` - A closure that defines the routes in this group
    ///
    /// # Returns
    /// Returns a `RouteResult<()>` indicating success or failure
    ///
    /// # Example
    /// ```rust
    /// app.group("/api/v1", |router| {
    ///     router.get("/users", users_handler);
    ///     router.post("/users", create_user_handler);
    /// });
    /// ```
    fn group(&mut self, base_path: &'static str, registry: impl Fn(&mut GroupRouter)) {
        // Validate base path format
        if !base_path.starts_with('/') {
            if let Some(handler) = &self.data_mut().error_handler {
                handler(NgynError::Route(
                    "Group base path must start with '/'".to_string(),
                ));
            }
            return;
        }

        // Create and configure group router
        let mut group = GroupRouter {
            data: PlatformData {
                base_path,
                ..Default::default()
            },
        };

        // Register routes in the group
        registry(&mut group);

        // Merge group router with main router
        if let Err(e) = self.data_mut().router.merge(group.data.router) {
            if let Some(handler) = &self.data_mut().error_handler {
                handler(NgynError::Route(format!(
                    "Failed to merge route group '{}': {}",
                    base_path, e
                )));
            }
        }
    }

    /// Adds a middleware to the application.
    ///
    /// Middleware functions are executed in the order they are added, for every request
    /// that matches the route they are attached to.
    ///
    /// # Arguments
    /// * `middleware` - The middleware implementation to add
    ///
    /// # Example
    /// ```rust
    /// app.use_middleware(LoggingMiddleware::new());
    /// ```
    fn use_middleware(&mut self, middleware: impl NgynMiddleware + 'static) {
        self.data_mut().add_middleware(Box::new(middleware))
    }

    /// Sets the application state that will be available to all route handlers.
    ///
    /// The state can be any type that implements `AppState` and will be shared
    /// across all routes using Arc.
    ///
    /// # Arguments
    /// * `state` - The state implementation to set
    ///
    /// # Example
    /// ```rust
    /// app.set_state(AppConfig { debug: true });
    /// ```
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

    fn mount(&mut self) -> &str {
        self.data_mut().base_path
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
