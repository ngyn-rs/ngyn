use ngyn::prelude::*;

pub struct TestMiddleware;

impl NgynMiddleware for TestMiddleware {
    async fn handle<'a, 'b>(_cx: &'a mut NgynContext, _response: &'b mut NgynResponse) {
        println!("middleware works");
    }
}
