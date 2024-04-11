// a context extends hashmap to provide some extra functionality
//

use std::collections::HashMap;
use hyper::{body::Incoming, Request};

/// Represents the value of a context in Ngyn
///
/// # Examples
///
/// ```
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
    pub request: Request<Incoming>,
    pub params: Vec<(String, String)>,
    store: HashMap<String, NgynContextValue>,
}

impl NgynContext {
    pub fn get(&self, key: &str) -> &NgynContextValue {
        let value = self.store.get(key.to_lowercase().trim());
        match value {
            Some(value) => value,
            None => &NgynContextValue::Empty,
        }
    }

    pub fn set(&mut self, key: &str, value: NgynContextValue) {
        self.store.insert(key.trim().to_lowercase(), value);
    }

    pub fn remove(&mut self, key: &str) {
        self.store.remove(key.to_lowercase().trim());
    }

    pub fn clear(&mut self) {
        self.store.clear();
    }

    pub fn len(&self) -> usize {
        self.store.len()
    }

    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    pub fn has(&self, key: &str) -> bool {
        self.store.contains_key(key.to_lowercase().trim())
    }

    pub fn is(&self, key: &str) -> bool {
        let stored_value = self.get(key);
        !stored_value.is_empty() || stored_value.clone().into()
    }

    pub fn from_request(request: Request<Incoming>) -> Self {
        NgynContext {
            request,
            store: HashMap::new(),
            params: Vec::new(),
        }
    }
}
