use ngyn::prelude::*;

pub struct TestMiddleware;

impl NgynMiddleware for TestMiddleware {
    async fn handle<'a>(_cx: &'a mut NgynContext, _response: &'a mut NgynResponse) {
        println!("middleware works");
    }
}
