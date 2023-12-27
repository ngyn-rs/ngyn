use ngyn::prelude::*;

#[injectable]
pub struct TestMiddleware;

impl NgynMiddleware for TestMiddleware {
    fn handle(&self, _request: &mut NgynRequest, _response: &mut NgynResponse) {
        println!("middleware works");
    }
}
