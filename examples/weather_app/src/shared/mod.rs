use http_body_util::BodyExt;
use ngyn::{
    prelude::*,
    shared::{server::ParseBytes, traits::NgynInterpreter},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct ErrorData {
    status: Option<u16>,
    message: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct CommonResponse {
    data: Option<Value>,
    error: Option<ErrorData>,
}

pub struct ResponseInterpreter {}

#[async_trait]
impl NgynInterpreter for ResponseInterpreter {
    async fn interpret(&self, res: &mut NgynResponse) -> NgynResponse {
        let (parts, mut body) = res.clone().into_parts();
        let body_str = {
            let mut buf = String::new();
            let frame = body.frame().await;

            if let Some(Ok(chunk)) = frame {
                buf = chunk.into_data().unwrap().parse_bytes();
            }
            buf
        };
        let response: CommonResponse = serde_json::from_str(&body_str).unwrap();
        let mut new_res = NgynResponse::from_parts(parts, body_str.into());
        if let Some(error) = response.error {
            if let Some(status) = error.status {
                new_res.set_status(status);
            }
        }
        new_res
    }
}
