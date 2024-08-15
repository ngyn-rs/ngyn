use ngyn::{prelude::*, shared::server::ToBytes};
use serde_json::json;

#[injectable]
pub struct NotFoundMiddleware;

impl NgynMiddleware for NotFoundMiddleware {
    fn handle(&self, cx: &mut NgynContext, res: &mut NgynResponse) {
        if cx.is_valid_route() {
            return;
        }
        *res.body_mut() = json!({
            "error": {
                "status": 404, // this will be interpreted by the ResponseInterpreter, and set as the status code
                "message": "Route not found",
            }
        })
        .to_bytes()
        .into();
    }
}
