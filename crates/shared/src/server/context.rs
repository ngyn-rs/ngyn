// a context extends hashmap to provide some extra functionality
//

use hyper::Request;
use std::{collections::HashMap, sync::Arc};

use crate::{uri::ToParams, Method, NgynController, NgynRequest, NgynResponse, Transformer};

/// Represents the value of a context in Ngyn
///
/// # Examples
///
/// ```rust ignore
/// use ngyn_shared::context::NgynContextValue;
///
/// let string_value: NgynContextValue = "Hello, world!".into();
/// let number_value: NgynContextValue = 42.into();
/// let bool_value: NgynContextValue = true.into();
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum NgynContextValue<V = String> {
    String(String),
    Number(i64),
    Boolean(bool),
    Vector(Vec<V>),
    Tuple(V),
    Empty,
}

impl NgynContextValue {
    /// Checks if the value is empty
    pub fn is_empty(&self) -> bool {
        match self {
            NgynContextValue::String(value) => value.is_empty(),
            NgynContextValue::Number(_) => false,
            NgynContextValue::Boolean(_) => false,
            NgynContextValue::Vector(value) => value.is_empty(),
            NgynContextValue::Tuple(_) => false,
            NgynContextValue::Empty => true,
        }
    }
}

impl From<String> for NgynContextValue {
    fn from(value: String) -> Self {
        NgynContextValue::String(value)
    }
}

impl From<&str> for NgynContextValue {
    fn from(value: &str) -> Self {
        NgynContextValue::String(value.to_string())
    }
}

impl From<bool> for NgynContextValue {
    fn from(value: bool) -> Self {
        NgynContextValue::Boolean(value)
    }
}

impl From<isize> for NgynContextValue {
    fn from(value: isize) -> Self {
        NgynContextValue::Number(value as i64)
    }
}

impl From<i64> for NgynContextValue {
    fn from(value: i64) -> Self {
        NgynContextValue::Number(value)
    }
}

impl From<i32> for NgynContextValue {
    fn from(value: i32) -> Self {
        NgynContextValue::Number(value as i64)
    }
}

impl From<usize> for NgynContextValue {
    fn from(value: usize) -> Self {
        NgynContextValue::Number(value as i64)
    }
}

impl<T> From<Vec<T>> for NgynContextValue<T> {
    fn from(value: Vec<T>) -> Self {
        NgynContextValue::Vector(value)
    }
}

impl From<NgynContextValue> for bool {
    fn from(value: NgynContextValue) -> Self {
        match value {
            NgynContextValue::Boolean(b) => b,
            _ => false, // or handle the conversion based on your specific logic
        }
    }
}

impl From<NgynContextValue> for String {
    fn from(value: NgynContextValue) -> Self {
        match value {
            NgynContextValue::String(val) => val,
            _ => panic!("conversion failed"),
        }
    }
}
pub struct NgynContext {
    pub request: Request<Vec<u8>>,
    pub params: Vec<(String, String)>,
    route_info: Option<(String, Arc<dyn NgynController>)>,
    store: HashMap<String, NgynContextValue>,
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
    /// use ngyn_shared::context::{NgynContext, NgynContextValue};
    ///
    /// let mut context = NgynContext::from_request(request);
    /// context.set("name", "John".into());
    ///
    /// let value = context.get("name");
    /// assert_eq!(value, &NgynContextValue::String("John".to_string()));
    /// ```
    pub fn get(&self, key: &str) -> &NgynContextValue {
        let value = self.store.get(key.to_lowercase().trim());
        match value {
            Some(value) => value,
            None => &NgynContextValue::Empty,
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
    /// use ngyn_shared::context::{NgynContext, NgynContextValue};
    ///
    /// let mut context = NgynContext::from_request(request);
    /// context.set("name", "John".into());
    ///
    /// let value = context.get("name");
    /// assert_eq!(value, &NgynContextValue::String("John".to_string()));
    /// ```
    pub fn set(&mut self, key: &str, value: NgynContextValue) {
        self.store.insert(key.trim().to_lowercase(), value);
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
    /// use ngyn_shared::context::{NgynContext, NgynContextValue};
    ///
    /// let mut context = NgynContext::from_request(request);
    /// context.set("name", "John".into());
    ///
    /// context.remove("name");
    /// let value = context.get("name");
    /// assert_eq!(value, &NgynContextValue::Empty);
    /// ```
    pub fn remove(&mut self, key: &str) {
        self.store.remove(key.to_lowercase().trim());
    }

    /// Clears all values from the context.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::context::{NgynContext, NgynContextValue};
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
    /// use ngyn_shared::context::{NgynContext, NgynContextValue};
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
    /// use ngyn_shared::context::{NgynContext, NgynContextValue};
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
    /// use ngyn_shared::context::{NgynContext, NgynContextValue};
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

    /// Checks if the value associated with the given key is not empty.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to check the value for.
    ///
    /// # Returns
    ///
    /// `true` if the value associated with the key is not empty, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::context::{NgynContext, NgynContextValue};
    ///
    /// let mut context = NgynContext::from_request(request);
    /// context.set("name", "John".into());
    ///
    /// assert!(context.is("name"));
    /// assert!(!context.is("age"));
    /// ```
    pub fn is(&self, key: &str) -> bool {
        let stored_value = self.get(key);
        !stored_value.is_empty() || stored_value.clone().into()
    }

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
    /// use ngyn_shared::context::{NgynContext, NgynContextValue};
    /// use hyper::Request;
    ///
    /// let request = Request::new(Vec::new());
    /// let context = NgynContext::from_request(request);
    /// assert!(context.is_empty());
    /// ```
    pub fn from_request(request: Request<Vec<u8>>) -> Self {
        NgynContext {
            request,
            store: HashMap::new(),
            params: Vec::new(),
            route_info: None,
        }
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
    /// use ngyn_shared::context::{NgynContext, NgynContextValue};
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
    pub fn with(&mut self, path: &str, method: &Method) -> Option<&mut Self> {
        if method.ne(self.request.method()) {
            return None;
        }
        if let Some(params) = self.request.uri().to_params(path) {
            self.params = params;
            Some(self)
        } else {
            None
        }
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
    /// use ngyn_shared::context::{NgynContext, NgynContextValue};
    /// use ngyn_shared::NgynController;
    ///
    /// let mut context = NgynContext::from_request(request);
    /// let controller = MyController::new();
    ///
    /// context.prepare(Arc::new(controller), "index".to_string());
    /// ```
    pub fn prepare(&mut self, controller: Arc<dyn NgynController>, handler: String) {
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
    /// use ngyn_shared::context::{NgynContext, NgynContextValue};
    /// use ngyn_shared::NgynResponse;
    ///
    /// let mut context = NgynContext::from_request(request);
    /// let mut response = NgynResponse::new();
    ///
    /// context.execute(&mut response).await;
    /// ```
    pub async fn execute(&mut self, res: &mut NgynResponse) {
        if let Some((handler, controller)) = self.route_info.clone() {
            controller.handle(handler.as_str(), self, res).await;
        }
    }
}

impl Transformer for NgynRequest {
    fn transform(cx: &mut NgynContext, _res: &mut NgynResponse) -> Self {
        cx.request.clone()
    }
}
