use http::Request;
use serde::{Deserialize, Serialize};
use std::{any::Any, collections::HashMap, mem::ManuallyDrop, sync::Arc};

use crate::{
    server::{uri::ToParams, Method, NgynRequest, NgynResponse, Transformer},
    traits::NgynController,
};

/// Represents the value of a context in Ngyn
#[derive(Serialize, Deserialize)]
struct NgynContextValue<V> {
    value: V,
}

impl<V> NgynContextValue<V> {
    pub fn create(value: V) -> Self {
        Self { value }
    }
}

/// Represents the state of an application in Ngyn

pub trait AppState: Any + Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: AppState> AppState for Box<T> {
    fn as_any(&self) -> &dyn Any {
        self.as_ref()
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self.as_mut()
    }
}

impl From<&Arc<Box<dyn AppState>>> for Box<dyn AppState> {
    fn from(value: &Arc<Box<dyn AppState>>) -> Self {
        let arc_clone = value.clone();
        let state_ref: &dyn AppState = &**arc_clone;

        let state_ptr: *const dyn AppState = state_ref as *const dyn AppState;

        let nn_ptr = std::ptr::NonNull::new(state_ptr as *mut dyn AppState).unwrap();
        let raw_ptr = nn_ptr.as_ptr();

        unsafe { Box::from_raw(raw_ptr) }
    }
}

/// Represents the context of a request in Ngyn
pub struct NgynContext {
    request: Request<Vec<u8>>,
    params: Option<Vec<(String, String)>>,
    route_info: Option<(String, Arc<Box<dyn NgynController>>)>,
    store: HashMap<String, String>,
    pub(crate) state: Option<Box<dyn AppState>>,
}

impl NgynContext {
    /// Retrieves the request associated with the context.
    ///
    /// ### Returns
    ///
    /// A reference to the request associated with the context.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::core::context::NgynContext;
    /// use hyper::Request;
    ///
    /// let request = Request::new(Vec::new());
    /// let context = NgynContext::from_request(request);
    ///
    /// let request_ref = context.request();
    /// ```
    pub fn request(&self) -> &Request<Vec<u8>> {
        &self.request
    }

    /// Retrieves the params associated with the context.
    ///
    /// ### Returns
    ///
    /// An optional reference to the params associated with the context.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::core::context::NgynContext;
    ///
    /// let mut context = NgynContext::from_request(request);
    /// context.set("name", "John".to_string());
    ///
    /// let params_ref = context.params();
    /// ```
    pub fn params(&self) -> Option<&Vec<(String, String)>> {
        self.params.as_ref()
    }
}

impl NgynContext {
    /// Retrieves the state of the context as a reference to the specified type.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of the state to retrieve.
    ///
    /// ### Returns
    ///
    /// An optional reference to the state of the specified type. Returns `None` if the state is not found or if it cannot be downcasted to the specified type.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::core::context::NgynContext;
    ///
    /// let mut context = NgynContext::from_request(request);
    ///
    /// let state_ref = context.state::<TestAppState>();
    /// ```
    pub fn state<T: 'static>(&self) -> Option<&T> {
        match &self.state {
            Some(value) => value.as_any().downcast_ref::<T>(),
            None => None,
        }
    }

    /// Retrieves the state of the context as a mutable reference to the specified type.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of the state to retrieve.
    ///
    /// ### Returns
    ///
    /// An optional reference to the state of the specified type. Returns `None` if the state is not found or if it cannot be downcasted to the specified type.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::core::context::NgynContext;
    ///
    /// let mut context = NgynContext::from_request(request);
    ///
    /// let state_ref = context.state::<TestAppState>();
    /// ```
    pub fn state_mut<T: 'static>(&mut self) -> Option<&mut T> {
        match &mut self.state {
            Some(value) => value.as_any_mut().downcast_mut::<T>(),
            None => None,
        }
    }
}

impl NgynContext {
    /// Retrieves the value associated with the given key from the context.
    ///
    /// ### Arguments
    ///
    /// * `key` - The key (case-insensitive) to retrieve the value for.
    ///
    /// ### Returns
    ///
    /// A reference to the value associated with the key. If the key is not found, returns a reference to an empty context value.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::core::context::NgynContext;
    ///
    /// let mut context = NgynContext::from_request(request);
    /// context.set("name", "John".to_string());
    ///
    /// let value: String = context.get("name").unwrap();
    /// assert_eq!(value, "John".to_string());
    /// ```
    pub fn get<V: for<'a> Deserialize<'a>>(&self, key: &str) -> Option<V> {
        let value = self.store.get(key.to_lowercase().trim());
        if let Some(value) = value {
            if let Ok(stored_cx) = serde_json::from_str::<NgynContextValue<V>>(value) {
                return Some(stored_cx.value);
            }
        }
        None
    }

    /// Sets the value associated with the given key in the context.
    ///
    /// ### Arguments
    ///
    /// * `key` - The key (case-insensitive) to set the value for.
    /// * `value` - The value to associate with the key.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::core::context::NgynContext;
    ///
    /// let mut context = NgynContext::from_request(request);
    /// context.set("name", "John".to_string());
    ///
    /// let value: String = context.get("name").unwrap();
    /// assert_eq!(value, "John".to_string());
    /// ```
    pub fn set<V: Serialize>(&mut self, key: &str, value: V) {
        if let Ok(value) = serde_json::to_string(&NgynContextValue::create(value)) {
            self.store.insert(key.trim().to_lowercase(), value);
        }
    }

    /// Removes the value associated with the given key from the context.
    ///
    /// ### Arguments
    ///
    /// * `key` - The key (case-insensitive) to remove the value for.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::core::context::NgynContext;
    ///
    /// let mut context = NgynContext::from_request(request);
    /// context.set("name", "John".to_string());
    ///
    /// context.remove("name");
    /// let value = context.get::<String>("name");
    /// assert_eq!(value, None);
    /// ```
    pub fn remove(&mut self, key: &str) {
        self.store.remove(key.to_lowercase().trim());
    }

    /// Clears all values from the context.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::core::context::NgynContext;
    ///
    /// let mut context = NgynContext::from_request(request);
    /// context.set("name", "John".to_string());
    /// context.set("age", 30.into());
    ///
    /// context.clear();
    /// assert_eq!(context.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.store.clear();
    }

    /// Returns the number of values in the context.
    ///
    /// ### Returns
    ///
    /// The number of values in the context.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::core::context::NgynContext;
    ///
    /// let mut context = NgynContext::from_request(request);
    /// context.set("name", "John".to_string());
    /// context.set("age", 30.into());
    ///
    /// assert_eq!(context.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.store.len()
    }

    /// Checks if the context is empty.
    ///
    /// ### Returns
    ///
    /// `true` if the context is empty, `false` otherwise.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::core::context::NgynContext;
    ///
    /// let mut context = NgynContext::from_request(request);
    ///
    /// assert!(context.is_empty());
    ///
    /// context.set("name", "John".to_string());
    /// assert!(!context.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    /// Checks if the context contains a value for the given key.
    ///
    /// ### Arguments
    ///
    /// * `key` - The key (case-insensitive) to check for.
    ///
    /// ### Returns
    ///
    /// `true` if the context contains a value for the key, `false` otherwise.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::core::context::NgynContext;
    ///
    /// let mut context = NgynContext::from_request(request);
    /// context.set("name", "John".to_string());
    ///
    /// assert!(context.has("name"));
    /// assert!(!context.has("age"));
    /// ```
    pub fn has(&self, key: &str) -> bool {
        self.store.contains_key(key.to_lowercase().trim())
    }
}

impl NgynContext {
    /// Checks if the context has a valid route.
    /// A valid route is when the route information and the params are set.
    /// This is great for differentiating known routes from unknown routes.
    ///
    /// ### Returns
    ///
    /// `true` if the context has a valid route, `false` otherwise.
    pub fn is_valid_route(&self) -> bool {
        self.params.is_some()
    }
}

impl NgynContext {
    /// Creates a new `NgynContext` from the given request.
    ///
    /// ### Arguments
    ///
    /// * `request` - The request to create the context from.
    ///
    /// ### Returns
    ///
    /// A new `NgynContext` instance.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::core::context::NgynContext;
    /// use hyper::Request;
    ///
    /// let request = Request::new(Vec::new());
    /// let context = NgynContext::from_request(request);
    /// assert!(context.is_empty());
    /// ```
    pub(crate) fn from_request(request: Request<Vec<u8>>) -> Self {
        NgynContext {
            request,
            store: HashMap::new(),
            params: None,
            route_info: None,
            state: None,
        }
    }

    /// Sets the route information for the context.
    ///
    /// ### Arguments
    ///
    /// * `path` - The path of the route.
    /// * `method` - The HTTP method of the route.
    ///
    /// ### Returns
    ///
    /// If the method of the request matches the given method and the path matches the route, returns a mutable reference to the context. Otherwise, returns `None`.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::core::context::NgynContext;
    /// use ngyn_shared::Method;
    ///
    /// let mut context = NgynContext::from_request(request);
    /// context.set("name", "John".to_string());
    ///
    /// let result = context.with("/users", &Method::GET);
    /// assert!(result.is_none());
    ///
    /// let result = context.with("/users", &Method::POST);
    /// assert!(result.is_some());
    /// ```
    pub(crate) fn with(&mut self, path: &str, method: Option<&Method>) -> Option<&mut Self> {
        if let Some(method) = method {
            if method != self.request.method()
            // HEAD is a GET request without a body
                || (method != Method::GET && self.request.method() != Method::HEAD)
            {
                return None;
            }
        }
        if let Some(params) = self.request.uri().to_params(path) {
            self.params = Some(params);
            Some(self)
        } else {
            None
        }
    }

    /// Prepares the context for execution by setting the route information.
    ///
    /// ### Arguments
    ///
    /// * `controller` - The controller to handle the request.
    /// * `handler` - The handler to execute.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::core::context::NgynContext;
    /// use ngyn_shared::NgynController;
    ///
    /// let mut context = NgynContext::from_request(request);
    /// let controller = MyController::new();
    ///
    /// context.prepare(Box::new(controller), "index".to_string());
    /// ```
    pub(crate) fn prepare(&mut self, controller: Arc<Box<dyn NgynController>>, handler: String) {
        self.route_info = Some((handler, controller));
    }

    /// Executes the handler associated with the route in the context.
    ///
    /// ### Arguments
    ///
    /// * `res` - The response to write to.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::core::context::NgynContext;
    /// use ngyn_shared::NgynResponse;
    ///
    /// let mut context = NgynContext::from_request(request);
    /// let mut response = NgynResponse::new();
    ///
    /// context.execute(&mut response).await;
    /// ```
    pub(crate) async fn execute(&mut self, res: &mut NgynResponse) {
        let (handler, controller) = match self.route_info.take() {
            Some((handler, ctrl)) => (handler, ctrl),
            None => return,
        };
        let mut controller =
            ManuallyDrop::<Box<dyn NgynController>>::new(controller.clone().into());
        controller.handle(&handler, self, res).await;
    }
}

impl<'a> Transformer<'a> for &'a NgynContext {
    fn transform(cx: &'a mut NgynContext, _res: &'a mut NgynResponse) -> Self {
        cx
    }
}

impl<'a> Transformer<'a> for &'a mut NgynContext {
    fn transform(cx: &'a mut NgynContext, _res: &'a mut NgynResponse) -> Self {
        cx
    }
}

impl<'a> Transformer<'a> for &'a NgynRequest {
    fn transform(cx: &'a mut NgynContext, _res: &'a mut NgynResponse) -> Self {
        cx.request()
    }
}

impl Transformer<'_> for NgynRequest {
    fn transform(cx: &mut NgynContext, _res: &mut NgynResponse) -> Self {
        cx.request().clone()
    }
}
#[cfg(test)]
mod tests {
    use http::StatusCode;

    use crate::traits::NgynInjectable;

    use super::*;

    struct TestAppState {
        value: u128,
    }
    impl AppState for TestAppState {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    struct TestController {}
    impl NgynInjectable for TestController {
        fn new() -> Self {
            Self {}
        }
    }
    impl NgynController for TestController {}

    #[test]
    fn test_request() {
        let request = Request::new(Vec::new());
        let context = NgynContext::from_request(request);

        let request_ref = context.request();
        assert_eq!(request_ref.method(), Method::GET);
    }

    #[test]
    fn test_state() {
        let request = Request::new(Vec::new());
        let mut context = NgynContext::from_request(request);

        let state_ref = context.state::<TestAppState>();
        assert!(state_ref.is_none());

        context.state = Some(Box::new(TestAppState { value: 1 }));

        let state_ref = context.state::<TestAppState>();
        assert!(state_ref.is_some());
    }

    #[test]
    fn test_state_mut() {
        let request = Request::new(Vec::new());
        let mut context = NgynContext::from_request(request);
        context.state = Some(Box::new(TestAppState { value: 1 }));

        let state_ref = context.state_mut::<TestAppState>();
        assert!(state_ref.is_some());

        state_ref.unwrap().value = 2;

        let state_ref = context.state::<TestAppState>();
        assert_eq!(state_ref.unwrap().value, 2);
    }

    #[test]
    fn test_box_state_impl() {
        let mut state = Box::new(TestAppState { value: 42 });

        // Test as_any
        let any_ref = state.as_any();
        let downcast_result = any_ref.downcast_ref::<TestAppState>();
        assert!(downcast_result.is_some());
        let result = downcast_result.unwrap();
        assert_eq!(result.value, 42);

        // Test as_any_mut
        let any_mut_ref = state.as_any_mut();
        let downcast_mut_result = any_mut_ref.downcast_mut::<TestAppState>();
        assert!(downcast_mut_result.is_some());
        downcast_mut_result.unwrap().value = 99;
        assert_eq!(state.value, 99);
    }

    #[test]
    fn test_get() {
        let request = Request::new(Vec::new());
        let mut context = NgynContext::from_request(request);
        context.set("name", "John".to_string());

        let value: String = context.get("name").unwrap();
        assert_eq!(value, "John".to_string());
    }

    #[test]
    fn test_set() {
        let request = Request::new(Vec::new());
        let mut context = NgynContext::from_request(request);
        context.set("name", "John".to_string());

        let value: String = context.get("name").unwrap();
        assert_eq!(value, "John".to_string());
    }

    #[test]
    fn test_remove() {
        let request = Request::new(Vec::new());
        let mut context = NgynContext::from_request(request);
        context.set("name", "John".to_string());

        context.remove("name");
        let value = context.get::<String>("name");
        assert_eq!(value, None);
    }

    #[test]
    fn test_clear() {
        let request = Request::new(Vec::new());
        let mut context = NgynContext::from_request(request);
        context.set("name", "John".to_string());
        context.set("age", 30);

        context.clear();
        assert_eq!(context.len(), 0);
    }

    #[test]
    fn test_len() {
        let request = Request::new(Vec::new());
        let mut context = NgynContext::from_request(request);
        context.set("name", "John".to_string());
        context.set("age", 30);

        assert_eq!(context.len(), 2);
    }

    #[test]
    fn test_is_empty() {
        let request = Request::new(Vec::new());
        let mut context = NgynContext::from_request(request);

        assert!(context.is_empty());

        context.set("name", "John".to_string());
        assert!(!context.is_empty());
    }

    #[test]
    fn test_has() {
        let request = Request::new(Vec::new());
        let mut context = NgynContext::from_request(request);
        context.set("name", "John".to_string());

        assert!(context.has("name"));
        assert!(!context.has("age"));
    }

    #[test]
    fn test_is_valid_route() {
        let request = Request::new(Vec::new());
        let mut context = NgynContext::from_request(request);
        context.set("name", "John".to_string());

        assert!(!context.is_valid_route());

        let params = vec![("param1".to_string(), "value1".to_string())];
        context.params = Some(params);

        assert!(context.is_valid_route());
    }

    #[test]
    fn test_with() {
        let mut request = Request::new(Vec::new());
        *request.method_mut() = Method::GET;
        *request.uri_mut() = "/users".parse().unwrap();

        let mut context = NgynContext::from_request(request);

        let path = "/users";
        let result = context.with(path, None);
        assert!(result.is_some());

        let path = "/users";
        let method = &Method::GET;
        let result = context.with(path, Some(method));
        assert!(result.is_some());

        let path = "/users";
        let method = &Method::POST;
        let result = context.with(path, Some(method));
        assert!(result.is_none());
    }

    #[test]
    fn test_params() {
        let mut request = Request::new(Vec::new());
        *request.uri_mut() = "/users/123".parse().unwrap();
        *request.method_mut() = Method::GET;

        let mut context = NgynContext::from_request(request);
        context.set("name", "John".to_string());

        let params_ref = context.params();
        assert_eq!(params_ref.is_none(), true);

        let route_path = "/users/<id>";
        context.with(route_path, Some(&Method::GET));

        let params_ref = context.params();
        assert_eq!(params_ref.is_some(), true);
        assert_eq!(params_ref.unwrap()[0].0, "id");
        assert_eq!(params_ref.unwrap()[0].1, "123");
    }

    #[test]
    fn test_prepare() {
        let request = Request::new(Vec::new());
        let mut context = NgynContext::from_request(request);
        let controller = Arc::new(Box::new(TestController::new()) as Box<dyn NgynController>);

        context.prepare(controller.clone(), "index".to_string());

        let (handler, ctrl) = context.route_info.unwrap();
        assert_eq!(handler, "index");
        assert_eq!(ctrl.prefix(), controller.prefix());
    }

    #[tokio::test]
    async fn test_execute() {
        let request = Request::new(Vec::new());
        let mut context = NgynContext::from_request(request);
        let mut response = NgynResponse::default();
        let controller = Arc::new(Box::new(TestController::new()) as Box<dyn NgynController>);

        assert_eq!(response.status(), StatusCode::OK);

        context.prepare(controller.clone(), "index".to_string());
        context.execute(&mut response).await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
