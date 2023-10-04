use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
/// An enum that represents an HTTP method.
/// It is used to specify the HTTP method of a route.
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Connect,
    Options,
    Trace,
    Patch,
    Unknown,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Delete => "DELETE",
            Self::Head => "HEAD",
            Self::Connect => "CONNECT",
            Self::Options => "OPTIONS",
            Self::Trace => "TRACE",
            Self::Patch => "PATCH",
            _ => panic!("Unknown HTTP method"),
        }
    }
}

impl From<&str> for HttpMethod {
    /// Takes in any string literal and returns the corresponding `HttpMethod` enum.
    /// If the string literal is not a valid HTTP method, it returns `HttpMethod::Unknown`.
    fn from(method: &str) -> Self {
        match method.to_uppercase().as_str() {
            "GET" => Self::Get,
            "POST" => Self::Post,
            "PUT" => Self::Put,
            "DELETE" => Self::Delete,
            "HEAD" => Self::Head,
            "CONNECT" => Self::Connect,
            "OPTIONS" => Self::Options,
            "TRACE" => Self::Trace,
            "PATCH" => Self::Patch,
            _ => Self::Unknown,
        }
    }
}

impl FromStr for HttpMethod {
    type Err = ();

    fn from_str(method: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(method))
    }
}

impl From<String> for HttpMethod {
    fn from(method: String) -> Self {
        HttpMethod::from(method.as_str())
    }
}

impl From<&String> for HttpMethod {
    fn from(method: &String) -> Self {
        HttpMethod::from(method.as_str())
    }
}

impl From<&&str> for HttpMethod {
    fn from(method: &&str) -> Self {
        HttpMethod::from(*method)
    }
}
