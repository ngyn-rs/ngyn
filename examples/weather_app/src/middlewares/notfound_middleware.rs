use ngyn::{prelude::*, shared::server::ToBytes};
use serde_json::json;

pub struct NotFoundMiddleware;

impl NgynMiddleware for NotFoundMiddleware {
    async fn handle(cx: &mut NgynContext<'_>) {
        if cx.params().is_none() {
            let body = json!({
                "error": {
                    "status": 404, // this will be interpreted by the ResponseInterpreter, and set as the status code
                    "message": "Route not found",
                }
            });
            *cx.response_mut().body_mut() = body.to_bytes().into();
        }
    }
}
