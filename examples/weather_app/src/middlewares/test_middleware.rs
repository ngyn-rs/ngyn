use ngyn::prelude::*;

pub struct TestMiddleware;

impl NgynMiddleware for TestMiddleware {
    async fn handle(_cx: &mut NgynContext<'_>) {
        println!("middleware works");
    }
}
