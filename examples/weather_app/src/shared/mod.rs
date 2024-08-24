use ngyn::{
    http::StatusCode,
    prelude::*,
    shared::{server::response::PeekBytes, traits::NgynInterpreter},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct ErrorData {
    status: Option<u16>,
    message: Option<String>,
}

type Res = JsonResponse<Value, ErrorData>;

pub struct ResponseInterpreter {}

#[async_trait]
impl NgynInterpreter for ResponseInterpreter {
    async fn interpret(&self, res: &mut NgynResponse) {
        let mut body_str = String::new();

        res.peek_bytes(|bytes| {
            body_str = String::from_utf8_lossy(bytes).to_string();
        })
        .await;

        if let Ok(response) = serde_json::from_str::<Res>(&body_str) {
            if let Some(error) = response.error() {
                if let Some(status) = error.status {
                    match StatusCode::from_u16(status) {
                        Ok(status) => {
                            *res.status_mut() = status;
                        }
                        Err(_) => {
                            println!("Seems like we set an invalid status code 🫠");
                            *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                        }
                    }
                }
            }
        }
    }
}
