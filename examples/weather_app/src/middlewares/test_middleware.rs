use ngyn::prelude::*;

#[injectable]
pub struct TestMiddleware;

impl NgynMiddleware<'_> for TestMiddleware {
    fn handle(&self, _cx: &mut NgynContext, _response: &mut NgynResponse) {
        println!("middleware works");
    }
}
