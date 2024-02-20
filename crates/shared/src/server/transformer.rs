use std::borrow::Cow;
use url::Url;

use crate::{NgynRequest, NgynResponse};

pub trait Transformer {
    fn transform(req: &mut NgynRequest, res: &mut NgynResponse) -> Self;
}

pub struct Transducer;

impl Transducer {
    #[allow(dead_code)]
    pub fn reduce<S: Transformer>(req: &mut NgynRequest, res: &mut NgynResponse) -> S {
        S::transform(req, res)
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
    fn transform(req: &mut NgynRequest, _res: &mut NgynResponse) -> Param {
        let data: Vec<(Cow<'static, str>, Cow<'static, str>)> = req
            .params()
            .iter()
            .map(|(key, value)| (Cow::Owned(key.to_string()), Cow::Owned(value.to_string())))
            .collect();
        Param { data }
    }
}

pub struct Query {
    url: Url,
}

impl Query {
    #[allow(dead_code)]
    pub fn get(&self, id: &str) -> Option<String> {
        self.url
            .query_pairs()
            .filter(|(key, _)| key == id)
            .map(|(_, value)| value.to_string())
            .next()
    }
}

impl Transformer for Query {
    fn transform(req: &mut NgynRequest, _res: &mut NgynResponse) -> Query {
        Query {
            url: req.url().clone(),
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
    fn transform(req: &mut NgynRequest, _res: &mut NgynResponse) -> Dto {
        let data = req.body_string().unwrap_or("{}".to_string());
        Dto { data }
    }
}
