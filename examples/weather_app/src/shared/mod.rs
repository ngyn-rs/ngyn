use http_body_util::BodyExt;
use ngyn::{prelude::*, shared::traits::NgynInterpreter};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct ErrorData {
    status: Option<u16>,
    message: Option<String>,
}

pub struct ResponseInterpreter {}

#[async_trait]
impl NgynInterpreter for ResponseInterpreter {
    async fn interpret(&self, res: &mut NgynResponse) {
        let mut body_str = String::new();

        let frame = res.frame().await;
        if let Some(Ok(frame)) = frame {
            if let Ok(bytes) = frame.into_data() {
                // Parse the body into a string.
                // This process may fail, since Ngyn allows any valid vec of bytes to be a body.
                // If the body cannot be parsed into a string, we will just ignore it.
                if let Ok(body) = &String::from_utf8_lossy(&bytes).parse::<String>() {
                    body_str.push_str(body);
                }
                // body has been read, so we need to set it back
                *res.body_mut() = bytes.into();
            }
        }

        if let Ok(response) = serde_json::from_str::<CommonResponse<Value, ErrorData>>(&body_str) {
            if let Some(error) = response.error {
                if let Some(status) = error.status {
                    res.set_status(status);
                }
            }
        }
    }
}
