use ngyn::{injectable, NgynMiddleware, NgynRequest, NgynResponse};

#[injectable]
pub struct TestMiddleware;

impl NgynMiddleware for TestMiddleware {
    fn handle(&self, _request: &NgynRequest, _response: &mut NgynResponse) {
        println!("middleware works");
    }
}
