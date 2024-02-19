use std::collections::HashMap;

use url::Url;

use super::context::NgynContext;
use crate::{enums::HttpMethod, transformer::Transformer, NgynResponse};

/// A struct that represents a request.
#[derive(Clone)]
pub struct NgynRequest {
    /// The HTTP method of the request.
    method: HttpMethod,
    /// The URL of the request.
    url: Url,
    /// The headers of the request.
    headers: HashMap<String, String>,
    /// body of the request in bytes format
    body: Option<Vec<u8>>,
    /// context for the request
    pub context: NgynContext,
    /// The parameters of the request.
    params: HashMap<String, String>,
}

impl NgynRequest {
    fn read_body(&mut self) -> Result<Vec<u8>, std::io::Error> {
        if let Some(body) = &self.body {
            return Ok(body.clone());
        }
        self.body = None;
        panic!("Body has already been read. Do you have more than one Dto in your route?");
    }

    /// Gets the body of the `NgynRequest`.
    pub fn body_string(&mut self) -> Result<String, std::io::Error> {
        Ok(String::from_utf8(self.read_body()?).unwrap())
    }

    /// Gets the HTTP method of the `NgynRequest`.
    pub fn method(&self) -> &HttpMethod {
        &self.method
    }

    /// Gets the URL of the `NgynRequest`.
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Gets the headers of the `NgynRequest`.
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn params(&self) -> &HashMap<String, String> {
		&self.params
	}

	pub fn set_params(&mut self, params: HashMap<String, String>) {
		if !self.params.is_empty() {
			panic!("Params have already been set");
		}
		self.params = params;
	}

	pub fn from_method(method: HttpMethod, url: &str, body: Option<Vec<u8>>, headers: HashMap<String, String>) -> Self {
		Self {
			method,
			url: Url::parse(url).unwrap(),
			headers,
			body,
			context: NgynContext::default(),
			params: HashMap::new(),
		}
	}

    pub fn from_get(url: &str, headers: HashMap<String, String>) -> Self {
		Self::from_method(HttpMethod::Get, url, None, headers)
	}

	pub fn from_post(url: &str, body: Vec<u8>, headers: HashMap<String, String>) -> Self {
		Self::from_method(HttpMethod::Post, url, Some(body), headers)
	}
}

impl From<(String, String, HashMap<String, String>, Vec<u8>)> for NgynRequest {
    fn from(value: (String, String, HashMap<String, String>, Vec<u8>)) -> Self {
        let (method, url, headers, body) = value;
        Self::from_method(method.into(), &url, Some(body), headers)
    }
}

impl Transformer for NgynRequest {
	fn transform(req: &mut NgynRequest, _res: &mut NgynResponse) -> Self {
		req.clone()
	}
}
