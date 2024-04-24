use std::borrow::Cow;

use crate::{context::NgynContext, NgynResponse};

pub trait Transformer {
    fn transform(cx: &mut NgynContext, res: &mut NgynResponse) -> Self;
}

pub struct Transducer;

impl Transducer {
    #[allow(dead_code)]
    pub fn reduce<S: Transformer>(cx: &mut NgynContext, res: &mut NgynResponse) -> S {
        S::transform(cx, res)
    }
}

pub struct Param {
    data: Vec<(Cow<'static, str>, Cow<'static, str>)>,
}

impl Param {
    #[allow(dead_code)]
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
    fn transform(cx: &mut NgynContext, _res: &mut NgynResponse) -> Param {
        let data: Vec<(Cow<'static, str>, Cow<'static, str>)> = cx
            .params
            .clone()
            .into_iter()
            .map(|(key, value)| (Cow::Owned(key.to_string()), Cow::Owned(value.to_string())))
            .collect();
        Param { data }
    }
}

pub struct Query {
    url: hyper::http::uri::Uri,
}

impl Query {
    #[allow(dead_code)]
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
    fn transform(cx: &mut NgynContext, _res: &mut NgynResponse) -> Query {
        Query {
            url: cx.request.uri().clone(),
        }
    }
}

pub struct Dto {
    data: String,
}

impl Dto {
    #[allow(dead_code)]
    pub fn parse<S: for<'a> serde::Deserialize<'a>>(&self) -> Result<S, serde_json::Error> {
        let data = self.data.as_str();
        serde_json::from_str(data)
    }
}

impl Transformer for Dto {
    fn transform(cx: &mut NgynContext, _res: &mut NgynResponse) -> Dto {
        let data = String::from_utf8_lossy(cx.request.body()).to_string();
        Dto { data }
    }
}
