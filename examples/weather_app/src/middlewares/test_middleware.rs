use ngyn::{injectable, NgynMiddleware};

#[injectable]
pub struct TestMiddleware;

impl NgynMiddleware for TestMiddleware {
    fn handle(&self, _request: &ngyn::NgynRequest, _response: &ngyn::NgynResponse) {
        println!("middleware works");
    }
}
