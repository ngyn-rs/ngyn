use crate::{context::NgynContext, NgynResponse};
use std::borrow::Cow;

/// Represents a transformer trait.
pub trait Transformer {
    /// Transforms the given `NgynContext` and `NgynResponse` and returns an instance of `Self`.
    ///
    /// # Arguments
    ///
    /// * `cx` - The mutable reference to the `NgynContext`.
    /// * `res` - The mutable reference to the `NgynResponse`.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// struct MyTransformer;
    ///
    /// impl Transformer for MyTransformer {
    ///     fn transform(cx: &mut NgynContext, res: &mut NgynResponse) -> Self {
    ///         // Transformation logic goes here
    ///         MyTransformer
    ///     }
    /// }
    /// ```
    fn transform(cx: &mut NgynContext, res: &mut NgynResponse) -> Self;
}

/// Represents a transducer struct.
pub struct Transducer;

impl Transducer {
    /// Reduces the given `NgynContext` and `NgynResponse` using the specified `Transformer` and returns an instance of `S`.
    ///
    /// # Arguments
    ///
    /// * `cx` - The mutable reference to the `NgynContext`.
    /// * `res` - The mutable reference to the `NgynResponse`.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    ///
    /// struct MyTransformer;
    ///
    /// impl Transformer for MyTransformer {
    ///     fn transform(cx: &mut NgynContext, res: &mut NgynResponse) -> Self {
    ///         // Transformation logic goes here
    ///         MyTransformer
    ///     }
    /// }
    ///
    /// let mut cx = NgynContext::new();
    /// let mut res = NgynResponse::new();
    ///
    /// let result: MyTransformer = Transducer::reduce(&mut cx, &mut res);
    /// ```
    pub fn reduce<S: Transformer>(cx: &mut NgynContext, res: &mut NgynResponse) -> S {
        S::transform(cx, res)
    }
}

/// Represents a parameter struct.
pub struct Param {
    data: Vec<(Cow<'static, str>, Cow<'static, str>)>,
}

impl Param {
    /// Retrieves the value associated with the specified `id` from the parameter data.
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier to search for.
    ///
    /// # Returns
    ///
    /// * `Some(String)` - The value associated with the `id`, if found.
    /// * `None` - If no value is associated with the `id`.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// let param = Param {
    ///     data: vec![
    ///         (Cow::Borrowed("id"), Cow::Borrowed("123")),
    ///         (Cow::Borrowed("name"), Cow::Borrowed("John")),
    ///     ],
    /// };
    ///
    /// assert_eq!(param.get("id"), Some("123".to_string()));
    /// assert_eq!(param.get("name"), Some("John".to_string()));
    /// assert_eq!(param.get("age"), None);
    /// ```
    pub fn get(&self, id: &str) -> Option<String> {
        for (key, value) in &self.data {
            if key == id {
                return Some(value.to_string());
            }
        }
        None
    }
}

impl Transformer for Param {
    /// Transforms the given `NgynContext` and `_res` into a `Param` instance.
    ///
    /// # Arguments
    ///
    /// * `cx` - The mutable reference to the `NgynContext`.
    /// * `_res` - The mutable reference to the `NgynResponse`.
    ///
    /// # Returns
    ///
    /// * `Param` - The transformed `Param` instance.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use crate::{context::NgynContext, NgynResponse};
    ///
    /// let mut cx = NgynContext::new();
    /// let mut res = NgynResponse::new();
    ///
    /// let param: Param = Param::transform(&mut cx, &mut res);
    /// ```
    fn transform(cx: &mut NgynContext, _res: &mut NgynResponse) -> Param {
        let data: Vec<(Cow<'static, str>, Cow<'static, str>)> = cx
            .params
            .clone()
            .unwrap()
            .into_iter()
            .map(|(key, value)| (Cow::Owned(key.to_string()), Cow::Owned(value.to_string())))
            .collect();
        Param { data }
    }
}

/// Represents a query struct.
pub struct Query {
    url: hyper::http::uri::Uri,
}

impl Query {
    /// Retrieves the value associated with the specified `id` from the query parameters.
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier to search for.
    ///
    /// # Returns
    ///
    /// * `Some(String)` - The value associated with the `id`, if found.
    /// * `None` - If no value is associated with the `id`.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use hyper::Uri;
    ///
    /// let uri: Uri = "https://example.com/?id=123&name=John".parse().unwrap();
    /// let query = Query { url: uri };
    ///
    /// assert_eq!(query.get("id"), Some("123".to_string()));
    /// assert_eq!(query.get("name"), Some("John".to_string()));
    /// assert_eq!(query.get("age"), None);
    /// ```
    pub fn get(&self, id: &str) -> Option<String> {
        let query = self.url.query().unwrap_or("");
        let query = url::form_urlencoded::parse(query.as_bytes());
        for (key, value) in query {
            if key == id {
                return Some(value.to_string());
            }
        }
        None
    }
}

impl Transformer for Query {
    /// Transforms the given `NgynContext` and `_res` into a `Query` instance.
    ///
    /// # Arguments
    ///
    /// * `cx` - The mutable reference to the `NgynContext`.
    /// * `_res` - The mutable reference to the `NgynResponse`.
    ///
    /// # Returns
    ///
    /// * `Query` - The transformed `Query` instance.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use crate::{context::NgynContext, NgynResponse};
    /// use hyper::Uri;
    ///
    /// let mut cx = NgynContext::new();
    /// let mut res = NgynResponse::new();
    ///
    /// let uri: Uri = "https://example.com/?id=123&name=John".parse().unwrap();
    /// cx.request.set_uri(uri);
    ///
    /// let query: Query = Query::transform(&mut cx, &mut res);
    /// ```
    fn transform(cx: &mut NgynContext, _res: &mut NgynResponse) -> Query {
        Query {
            url: cx.request.uri().clone(),
        }
    }
}

/// Represents a data transfer object struct.
pub struct Dto {
    data: String,
}

impl Dto {
    /// Parses the data into the specified type using serde deserialization.
    ///
    /// # Arguments
    ///
    /// * `S` - The type to deserialize the data into.
    ///
    /// # Returns
    ///
    /// * `Result<S, serde_json::Error>` - The deserialized result, if successful.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use serde::Deserialize;
    ///
    /// let dto = Dto {
    ///     data: r#"{"name": "John", "age": 30}"#.to_string(),
    /// };
    ///
    /// #[derive(Deserialize)]
    /// struct Person {
    ///     name: String,
    ///     age: u32,
    /// }
    ///
    /// let result: Result<Person, serde_json::Error> = dto.parse();
    /// ```
    pub fn parse<S: for<'a> serde::Deserialize<'a>>(&self) -> Result<S, serde_json::Error> {
        let data = self.data.as_str();
        serde_json::from_str(data)
    }
}

impl Transformer for Dto {
    /// Transforms the given `NgynContext` and `_res` into a `Dto` instance.
    ///
    /// # Arguments
    ///
    /// * `cx` - The mutable reference to the `NgynContext`.
    /// * `_res` - The mutable reference to the `NgynResponse`.
    ///
    /// # Returns
    ///
    /// * `Dto` - The transformed `Dto` instance.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use crate::{context::NgynContext, NgynResponse};
    ///
    /// let mut cx = NgynContext::new();
    /// let mut res = NgynResponse::new();
    ///
    /// let dto: Dto = Dto::transform(&mut cx, &mut res);
    /// ```
    fn transform(cx: &mut NgynContext, _res: &mut NgynResponse) -> Dto {
        let data = String::from_utf8_lossy(cx.request.body()).to_string();
        Dto { data }
    }
}
