use ngyn::{prelude::*, shared::server::ToBytes};
use serde_json::json;

pub struct NotFoundMiddleware;

impl NgynMiddleware for NotFoundMiddleware {
    async fn handle<'a, 'b>(cx: &'a mut NgynContext, res: &'b mut NgynResponse) {
        if cx.params().is_none() {
            let body = json!({
                "error": {
                    "status": 404, // this will be interpreted by the ResponseInterpreter, and set as the status code
                    "message": "Route not found",
                }
            });
            *res.body_mut() = body.to_bytes().into();
        }
    }
}
