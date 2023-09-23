use ngyn::{injectable, NgynMiddleware};

#[injectable]
pub struct TestMiddleware;

impl NgynMiddleware for TestMiddleware {
    fn handle(&self, request: ngyn::NgynRequest, response: ngyn::NgynResponse, next: ngyn::NextFn) {
        print!("middleware works");
        next(request, response);
    }
}
