use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum NgynError {
    /// Represents routing errors (404, invalid path, etc)
    Route(String),
    /// Represents middleware errors
    Middleware(String),
    /// Represents handler errors
    Handler(String),
    /// Represents state errors
    State(String),
    /// Wraps other errors
    Other(Box<dyn Error + Send + Sync>),
}

impl fmt::Display for NgynError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NgynError::Route(msg) => write!(f, "Routing error: {}", msg),
            NgynError::Middleware(msg) => write!(f, "Middleware error: {}", msg),
            NgynError::Handler(msg) => write!(f, "Handler error: {}", msg),
            NgynError::State(msg) => write!(f, "State error: {}", msg),
            NgynError::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}

impl Error for NgynError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            NgynError::Other(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}

impl From<Box<dyn Error + Send + Sync>> for NgynError {
    fn from(error: Box<dyn Error + Send + Sync>) -> Self {
        NgynError::Other(error)
    }
}