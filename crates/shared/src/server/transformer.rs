use bytes::Bytes;
use futures_util::StreamExt;
use http::{header::CONTENT_TYPE, HeaderValue};
use http_body_util::{BodyStream, Full};
use multer::Multipart;
use serde::Deserialize;

use crate::server::NgynContext;

/// Represents a transformer trait.
pub trait Transformer<'a> {
    /// Transforms the given `NgynContext` and returns an instance of `Self`.
    ///
    /// ### Arguments
    ///
    /// * `cx` - The mutable reference to the `NgynContext`.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    /// struct MyTransformer;
    ///
    /// impl Transformer for MyTransformer {
    ///     fn transform(cx: &mut NgynContext) -> Self {
    ///         // Transformation logic goes here
    ///         MyTransformer
    ///     }
    /// }
    /// ```
    #[must_use]
    fn transform(cx: &'a mut NgynContext) -> Self
    where
        Self: Sized;
}

/// Represents a transducer struct.
pub struct Transducer;

impl<'a> Transducer {
    /// Reduces the given `NgynContext` using the specified `Transformer` and returns an instance of `S`.
    ///
    /// ### Arguments
    ///
    /// * `cx` - The mutable reference to the `NgynContext`.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    ///
    /// struct MyTransformer;
    ///
    /// impl Transformer for MyTransformer {
    ///     fn transform(cx: &mut NgynContext) -> Option<Self> {
    ///         // Transformation logic goes here
    ///         MyTransformer
    ///     }
    /// }
    ///
    /// let mut cx = NgynContext::default();
    ///
    /// let result: MyTransformer = Transducer::reduce(&mut cx);
    /// ```
    #[must_use]
    pub fn reduce<S: Transformer<'a>>(cx: &'a mut NgynContext) -> S {
        S::transform(cx)
    }
}

/// Represents a parameter struct.
pub struct Param<'a> {
    data: Vec<(&'a str, &'a str)>,
}

impl<'a> Param<'a> {
    /// Retrieves the value associated with the specified `id` from the parameter data.
    ///
    /// ### Arguments
    ///
    /// * `id` - The identifier to search for.
    ///
    /// ### Returns
    ///
    /// * `Some(String)` - The value associated with the `id`, if found.
    /// * `None` - If no value is associated with the `id`.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    /// let param = Param {
    ///     data: vec![
    ///         ("id", "123"),
    ///         ("name", "John"),
    ///     ],
    /// };
    ///
    /// assert_eq!(param.get("id"), Some("123".to_string()));
    /// assert_eq!(param.get("name"), Some("John".to_string()));
    /// assert_eq!(param.get("age"), None);
    /// ```
    pub fn get(&self, id: &str) -> Option<String> {
        for (key, value) in &self.data {
            if *key == id {
                return Some(value.to_string());
            }
        }
        None
    }
}

impl<'a: 'b, 'b> Transformer<'a> for Param<'b> {
    /// Transforms the given `NgynContext` into a `Param` instance.
    ///
    /// ### Arguments
    ///
    /// * `cx` - The mutable reference to the `NgynContext`.
    ///
    /// ### Returns
    ///
    /// * `Param` - The transformed `Param` instance.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    /// use crate::context::NgynContext;
    ///
    /// let mut cx = NgynContext::default();
    /// let param: Param = Param::transform(&mut cx);
    /// ```
    fn transform(cx: &'a mut NgynContext) -> Self {
        let data: Vec<(&'a str, &'a str)> = cx
            .params()
            .unwrap_or_else(|| panic!("Extracting params should only be done in route handlers.")) // Infallible, only fails if the route is invalid
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
            .collect();
        Param { data }
    }
}

/// Represents a query struct.
pub struct Query<'q> {
    uri: &'q http::uri::Uri,
}

impl<'q> Query<'q> {
    /// Retrieves the value associated with the specified `id` from the query parameters.
    ///
    /// ### Arguments
    ///
    /// * `id` - The identifier to search for.
    ///
    /// ### Returns
    ///
    /// * `Some(String)` - The value associated with the `id`, if found.
    /// * `None` - If no value is associated with the `id`.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    /// use hyper::Uri;
    ///
    /// let uri: Uri = "https://example.com/?id=123&name=John".parse().unwrap();
    /// let query = Query { uri: &uri };
    ///
    /// assert_eq!(query.get("id"), Some("123".to_string()));
    /// assert_eq!(query.get("name"), Some("John".to_string()));
    /// assert_eq!(query.get("age"), None);
    /// ```
    pub fn get(&self, id: &str) -> Option<String> {
        let query = self.uri.query().unwrap_or("");
        let query = url::form_urlencoded::parse(query.as_bytes());
        for (key, value) in query {
            if key == id {
                return Some(value.to_string());
            }
        }
        None
    }
}

impl<'a: 'q, 'q> Transformer<'a> for Query<'q> {
    /// Transforms the given `NgynContext` into a `Query` instance.
    ///
    /// ### Arguments
    ///
    /// * `cx` - The mutable reference to the `NgynContext`.
    ///
    /// ### Returns
    ///
    /// * `Query` - The transformed `Query` instance.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    /// use crate::context::NgynContext;
    /// use hyper::Uri;
    ///
    /// let mut cx = NgynContext::default();
    ///
    /// let uri: Uri = "https://example.com/?id=123&name=John".parse().unwrap();
    /// cx.request.set_uri(uri);
    ///
    /// let query: Query = Query::transform(&mut cx);
    /// ```
    fn transform(cx: &'a mut NgynContext) -> Self {
        Query {
            uri: cx.request().uri(),
        }
    }
}

/// Represents a data transfer object struct.
pub struct Body<'b> {
    content_type: Option<&'b HeaderValue>,
    data: &'b Vec<u8>,
}

impl<'b> Body<'b> {
    /// Parses the data into the specified type using serde deserialization.
    /// Once read, the body data is consumed and cannot be read again.
    ///
    /// ### Arguments
    ///
    /// * `S` - The type to deserialize the data into.
    ///
    /// ### Returns
    ///
    /// * `Result<S, serde_json::Error>` - The deserialized result, if successful.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    /// use serde::Deserialize;
    ///
    /// let body = Body {
    ///     data: r#"{"name": "John", "age": 30}"#.to_string().into_bytes(),
    /// };
    ///
    /// #[derive(Deserialize)]
    /// struct Person {
    ///     name: String,
    ///     age: u32,
    /// }
    ///
    /// let result: Result<Person, serde_json::Error> = body.json();
    /// ```
    pub fn json<S: for<'a> Deserialize<'a>>(self) -> Result<S, serde_json::Error> {
        serde_json::from_str(&self.text())
    }

    /// Reads the body data as a string.
    /// Once read, the body data is consumed and cannot be read again.
    ///
    /// ### Returns
    ///
    /// * `String` - The body data as a string.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    /// let body = Body {
    ///    data: r#"{"name": "John", "age": 30}"#.to_string().into_bytes(),
    /// };
    ///
    /// assert_eq!(body.text(), r#"{"name": "John", "age": 30}"#);
    /// ```
    pub fn text(self) -> String {
        String::from_utf8_lossy(self.data).to_string()
    }

    /// Parses the data into a `multipart/form-data` stream.
    /// Once read, the body data is consumed and cannot be read again.
    ///
    /// ### Returns
    ///
    /// * `Multipart<'static>` - The body data as a `multipart/form-data` stream.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    /// let body = Body {
    ///    data: r#"------WebKitFormBoundary7MA4YWxkTrZu0gW\r\nContent-Disposition: form-data; name="file"; filename="example.txt"\r\nContent-Type: text/plain\r\n\r\nHello World\r\n------WebKitFormBoundary7MA4YWxkTrZu0gW--\r\n"#.to_string().into_bytes(),
    /// };
    ///
    /// let stream = body.form_data();
    /// ```
    pub fn form_data(self) -> Result<Multipart<'b>, multer::Error> {
        if let Some(content_type) = self.content_type {
            let boundary = multer::parse_boundary(
                content_type
                    .to_str()
                    .expect("Content Type header contains invalid ASCII value"),
            )?;
            let body: Full<Bytes> = Full::new(Bytes::from(self.data.to_owned()));
            let stream = BodyStream::new(body).filter_map(|result| async move {
                result.map(|frame| frame.into_data().ok()).transpose()
            });
            Ok(Multipart::new(stream, boundary))
        } else {
            Err(multer::Error::NoBoundary)
        }
    }
}

impl<'a: 'b, 'b> Transformer<'a> for Body<'b> {
    /// Transforms the given `NgynContext` into a `Body` instance.
    ///
    /// ### Arguments
    ///
    /// * `cx` - The mutable reference to the `NgynContext`.
    ///
    /// ### Returns
    ///
    /// * `Body` - The transformed `Body` instance.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    /// use crate::context::NgynContext;
    ///
    /// let mut cx = NgynContext::default();
    ///
    /// let dto: Body = Body::transform(&mut cx);
    /// ```
    fn transform(cx: &'a mut NgynContext) -> Self {
        let data = cx.request().body();
        let content_type = cx.request().headers().get(CONTENT_TYPE);
        Body { data, content_type }
    }
}
