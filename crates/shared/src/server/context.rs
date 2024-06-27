// a context extends hashmap to provide some extra functionality
//

use hyper::Request;
use serde::{Deserialize, Serialize};
use std::{any::Any, collections::HashMap, sync::Arc};

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
pub trait AppState: Any + Send {
    fn as_any(&self) -> &dyn Any
    where
        Self: Sized,
    {
        self
    }
    fn get_state(&self) -> &dyn Any;
}

impl<T: Send + Sized + 'static> AppState for T {
    fn get_state(&self) -> &dyn Any {
        self
    }
}

/// Represents the context of a request in Ngyn
pub struct NgynContext {
    request: Request<Vec<u8>>,
    params: Option<Vec<(String, String)>>,
    route_info: Option<(String, Arc<dyn NgynController>)>,
    store: HashMap<String, String>,
    state: Option<Box<dyn AppState>>,
}

impl NgynContext {
    /// Retrieves the request associated with the context.
    ///
    /// # Returns
    ///
    /// A reference to the request associated with the context.
    ///
    /// # Examples
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
    /// # Returns
    ///
    /// An optional reference to the params associated with the context.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::core::context::NgynContext;
    ///
    /// let mut context = NgynContext::from_request(request);
    /// context.set("name", "John".into());
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
    /// # Returns
    ///
    /// An optional reference to the state of the specified type. Returns `None` if the state is not found or if it cannot be downcasted to the specified type.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::core::context::NgynContext;
    ///
    /// let mut context = NgynContext::from_request(request);
    /// context.set_state(Box::new(MyAppState::new()));
    ///
    /// let state_ref = context.state::<MyAppState>();
    /// ```
    pub fn state<T: 'static>(&self) -> Option<&T> {
        let state = self.state.as_ref();

        match state {
            Some(value) => value.get_state().downcast_ref::<T>(),
            None => None,
        }
    }
}

impl NgynContext {
    /// Retrieves the value associated with the given key from the context.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to retrieve the value for.
    ///
    /// # Returns
    ///
    /// A reference to the value associated with the key. If the key is not found, returns a reference to an empty context value.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::core::context::NgynContext;
    ///
    /// let mut context = NgynContext::from_request(request);
    /// context.set("name", "John".into());
    ///
    /// let value: String = context.get("name").unwrap();
    /// assert_eq!(value, "John".to_string());
    /// ```
    pub fn get<V: for<'a> Deserialize<'a>>(&self, key: &str) -> Option<V> {
        let value = self.store.get(key.to_lowercase().trim());
        match value {
            Some(v) => {
                let stored_cx: NgynContextValue<V> = serde_json::from_str(v).unwrap();
                Some(stored_cx.value)
            }
            None => None,
        }
    }

    /// Sets the value associated with the given key in the context.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to set the value for.
    /// * `value` - The value to associate with the key.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::core::context::NgynContext;
    ///
    /// let mut context = NgynContext::from_request(request);
    /// context.set("name", "John".into());
    ///
    /// let value: String = context.get("name").unwrap();
    /// assert_eq!(value, "John".to_string());
    /// ```
    pub fn set<V: Serialize>(&mut self, key: &str, value: V) {
        self.store.insert(
            key.trim().to_lowercase(),
            serde_json::to_string(&NgynContextValue::create(value)).unwrap(),
        );
    }

    /// Removes the value associated with the given key from the context.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to remove the value for.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::core::context::NgynContext;
    ///
    /// let mut context = NgynContext::from_request(request);
    /// context.set("name", "John".into());
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
    /// # Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::core::context::NgynContext;
    ///
    /// let mut context = NgynContext::from_request(request);
    /// context.set("name", "John".into());
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
    /// # Returns
    ///
    /// The number of values in the context.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::core::context::NgynContext;
    ///
    /// let mut context = NgynContext::from_request(request);
    /// context.set("name", "John".into());
    /// context.set("age", 30.into());
    ///
    /// assert_eq!(context.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.store.len()
    }

    /// Checks if the context is empty.
    ///
    /// # Returns
    ///
    /// `true` if the context is empty, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::core::context::NgynContext;
    ///
    /// let mut context = NgynContext::from_request(request);
    ///
    /// assert!(context.is_empty());
    ///
    /// context.set("name", "John".into());
    /// assert!(!context.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    /// Checks if the context contains a value for the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to check for.
    ///
    /// # Returns
    ///
    /// `true` if the context contains a value for the key, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::core::context::NgynContext;
    ///
    /// let mut context = NgynContext::from_request(request);
    /// context.set("name", "John".into());
    ///
    /// assert!(context.has("name"));
    /// assert!(!context.has("age"));
    /// ```
    pub fn has(&self, key: &str) -> bool {
        self.store.contains_key(key.to_lowercase().trim())
    }
}

impl NgynContext {
    /// Creates a new `NgynContext` from the given request.
    ///
    /// # Arguments
    ///
    /// * `request` - The request to create the context from.
    ///
    /// # Returns
    ///
    /// A new `NgynContext` instance.
    ///
    /// # Examples
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

    pub(crate) fn set_state(&mut self, state: Box<dyn AppState>) {
        self.state = Some(state);
    }

    /// Sets the route information for the context.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the route.
    /// * `method` - The HTTP method of the route.
    ///
    /// # Returns
    ///
    /// If the method of the request matches the given method and the path matches the route, returns a mutable reference to the context. Otherwise, returns `None`.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::core::context::NgynContext;
    /// use ngyn_shared::Method;
    ///
    /// let mut context = NgynContext::from_request(request);
    /// context.set("name", "John".into());
    ///
    /// let result = context.with("/users", &Method::GET);
    /// assert!(result.is_none());
    ///
    /// let result = context.with("/users", &Method::POST);
    /// assert!(result.is_some());
    /// ```
    pub(crate) fn with(&mut self, path: &str, method: &Method) -> Option<&mut Self> {
        if method.ne(self.request.method()) {
            return None;
        }
        if let Some(params) = self.request.uri().to_params(path) {
            self.params = Some(params);
            Some(self)
        } else {
            None
        }
    }

    /// Checks if the context has a valid route.
    /// A valid route is when the route information and the params are set.
    /// This is great for differentiating known routes from unknown routes.
    ///
    /// # Returns
    ///
    /// `true` if the context has a valid route, `false` otherwise.
    pub fn is_valid_route(&self) -> bool {
        self.route_info.is_some() && self.params.is_some()
    }

    /// Prepares the context for execution by setting the route information.
    ///
    /// # Arguments
    ///
    /// * `controller` - The controller to handle the request.
    /// * `handler` - The handler to execute.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::core::context::NgynContext;
    /// use ngyn_shared::NgynController;
    ///
    /// let mut context = NgynContext::from_request(request);
    /// let controller = MyController::new();
    ///
    /// context.prepare(Arc::new(controller), "index".to_string());
    /// ```
    pub(crate) fn prepare(&mut self, controller: Arc<dyn NgynController>, handler: String) {
        self.route_info = Some((handler, controller));
    }

    /// Executes the handler associated with the route in the context.
    ///
    /// # Arguments
    ///
    /// * `res` - The response to write to.
    ///
    /// # Examples
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
        if let Some((handler, controller)) = self.route_info.clone() {
            controller.inject(self);
            controller.handle(handler.as_str(), self, res).await;
        }
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
