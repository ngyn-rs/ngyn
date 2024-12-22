use http::Request;
use matchit::Params;
use serde::{Deserialize, Serialize};
use std::{any::Any, collections::HashMap, sync::Arc};

use crate::server::{NgynRequest, NgynResponse, Transformer};

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

/// # Panics
/// Panics if the state has been dropped. This should never happen unless the state is dropped manually.
impl From<&Arc<Box<dyn AppState>>> for Box<dyn AppState> {
    fn from(value: &Arc<Box<dyn AppState>>) -> Self {
        // creating a clone is essential since this ref will be dropped after this function returns
        let arc_clone = value.clone();
        let state_ref: &dyn AppState = &**arc_clone;

        let state_ptr: *const dyn AppState = state_ref as *const dyn AppState;

        // SAFETY: state_ptr is never null, it is safe to convert it to a NonNull pointer, this way we can safely convert it back to a Box
        // If it is ever found as null, this is a bug. It probably means the memory has been poisoned
        let nn_ptr = std::ptr::NonNull::new(state_ptr as *mut dyn AppState)
            .expect("State has been dropped, but this should never happen, ensure it is being cloned correctly."); // This should never happen, if it does, it's a bug
        let raw_ptr = nn_ptr.as_ptr();

        unsafe { Box::from_raw(raw_ptr) }
    }
}

/// Represents the context of a request in Ngyn
pub struct NgynContext<'a> {
    request: Request<Vec<u8>>,
    pub(crate) response: NgynResponse,
    pub(crate) params: Option<Params<'a, 'a>>,
    store: HashMap<&'a str, String>,
    pub(crate) state: Option<Box<dyn AppState>>,
}

impl<'a> NgynContext<'a> {
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

    #[deprecated(since="0.5.1", note="use `response_mut()` instead")]
    pub fn response(&mut self) -> &mut NgynResponse {
        &mut self.response
    }
    
    
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
    pub fn response_mut(&mut self) -> &mut NgynResponse {
        &mut self.response
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
    pub fn params(&self) -> Option<&Params<'a, 'a>> {
        self.params.as_ref()
    }
}

impl NgynContext<'_> {
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

impl<'b> NgynContext<'b> {
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
    pub fn set<V: Serialize>(&mut self, key: &'b str, value: V) {
        if let Ok(value) = serde_json::to_string(&NgynContextValue::create(value)) {
            self.store.insert(key.trim(), value);
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

impl NgynContext<'_> {
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
            response: NgynResponse::default(),
            store: HashMap::new(),
            params: None,
            state: None,
        }
    }
}

impl<'a> Transformer<'a> for &'a NgynContext<'a> {
    fn transform(cx: &'a mut NgynContext) -> Self {
        cx
    }
}

// impl<'a: 'b, 'b> Transformer<'a> for &'a mut NgynContext<'b> {
//     fn transform(cx: &'a mut NgynContext) -> Self {
//         cx
//     }
// }

impl<'a> Transformer<'a> for &'a NgynRequest {
    fn transform(cx: &'a mut NgynContext) -> Self {
        cx.request()
    }
}

impl Transformer<'_> for NgynRequest {
    fn transform(cx: &mut NgynContext) -> Self {
        cx.request().clone()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use http::Method;

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
}
