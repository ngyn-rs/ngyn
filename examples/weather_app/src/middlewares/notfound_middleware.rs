use ngyn::prelude::*;

#[injectable]
pub struct NotFoundMiddleware;

impl NgynMiddleware<'_> for NotFoundMiddleware {
    fn handle(&self, cx: &mut NgynContext, res: &mut NgynResponse) {
        if cx.is_valid_route() {
            return;
        }
        res.set_status(404);
        res.send("Not Found".to_string());
    }
}
