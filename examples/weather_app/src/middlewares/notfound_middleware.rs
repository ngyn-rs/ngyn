use ngyn::prelude::*;
use serde_json::json;

#[injectable]
pub struct NotFoundMiddleware;

impl NgynMiddleware for NotFoundMiddleware {
    fn handle(&self, cx: &mut NgynContext, res: &mut NgynResponse) {
        if cx.is_valid_route() {
            return;
        }
        res.send(json!({
            "error": {
                "status": 404,
                "message": "Route not found",
            }
        }));
    }
}
