use std::str::FromStr;

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
        }
    }
}

impl From<&str> for HttpMethod {
    fn from(method: &str) -> Self {
        match method {
            "GET" => Self::Get,
            "POST" => Self::Post,
            "PUT" => Self::Put,
            "DELETE" => Self::Delete,
            "HEAD" => Self::Head,
            "CONNECT" => Self::Connect,
            "OPTIONS" => Self::Options,
            "TRACE" => Self::Trace,
            "PATCH" => Self::Patch,
            _ => Self::Get,
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
