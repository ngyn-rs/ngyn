#[derive(Clone, PartialEq)]
/// Represents the body of a response in Ngyn
///
/// # Examples
///
/// ```
/// use ngyn_shared::NgynBody;
///
/// let string_body = NgynBody::String("Hello, world!".to_string());
/// let bool_body = NgynBody::Bool(true);
/// let number_body = NgynBody::Number(42);
/// let none_body = NgynBody::None;
/// ```
pub enum NgynBody {
    /// Represents a string body
    String(String),
    /// Represents a boolean body
    Bool(bool),
    /// Represents a numeric body
    Number(usize),
    /// Represents no body
    None,
}

impl From<String> for NgynBody {
    fn from(value: String) -> Self {
        NgynBody::String(value)
    }
}

impl From<bool> for NgynBody {
    fn from(value: bool) -> Self {
        NgynBody::Bool(value)
    }
}

impl From<usize> for NgynBody {
    fn from(value: usize) -> Self {
        NgynBody::Number(value)
    }
}

impl From<isize> for NgynBody {
    fn from(value: isize) -> Self {
        NgynBody::Number(value as usize)
    }
}

impl From<&str> for NgynBody {
    fn from(value: &str) -> Self {
        NgynBody::String(value.to_string())
    }
}

impl From<()> for NgynBody {
    fn from(_: ()) -> Self {
        NgynBody::None
    }
}
