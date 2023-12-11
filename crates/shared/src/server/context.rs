// a context extends hashmap to provide some extra functionality
//

use std::collections::HashMap;

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

impl Into<String> for NgynContextValue {
    fn into(self) -> String {
        match self {
            NgynContextValue::String(value) => value,
            NgynContextValue::Number(value) => value.to_string(),
            NgynContextValue::Boolean(value) => value.to_string(),
            NgynContextValue::Empty => "".to_string(),
            _ => panic!("Cannot convert vector or tuple to string"),
        }
    }
}

impl Into<i64> for NgynContextValue {
    fn into(self) -> i64 {
        match self {
            NgynContextValue::String(value) => value.parse::<i64>().unwrap(),
            NgynContextValue::Number(value) => value,
            NgynContextValue::Boolean(value) => value as i64,
            NgynContextValue::Empty => 0,
            _ => panic!("Cannot convert vector or tuple to number"),
        }
    }
}

impl Into<bool> for NgynContextValue {
    fn into(self) -> bool {
        match self {
            NgynContextValue::String(value) => value.trim() == "true",
            NgynContextValue::Number(value) => value == 1,
            NgynContextValue::Boolean(value) => value,
            NgynContextValue::Empty => false,
            _ => true,
        }
    }
}

impl<V> Into<Vec<V>> for NgynContextValue<V> {
    fn into(self) -> Vec<V> {
        match self {
            NgynContextValue::Vector(value) => value,
            NgynContextValue::Empty => vec![],
            _ => panic!("Cannot convert non-vector to vector"),
        }
    }
}

#[derive(Clone)]
pub struct NgynContext {
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
}

impl Default for NgynContext {
    fn default() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}
