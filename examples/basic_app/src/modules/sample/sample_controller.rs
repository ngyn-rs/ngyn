use ngyn::prelude::*;

use super::sample_service::SampleService;

#[controller]
pub struct SampleController {
    sample_service: SampleService,
}

#[routes]
impl SampleController {
    #[get("/hello")]
    fn say_hello(&self, _req: &NgynRequest, res: &mut NgynResponse) {
        self.sample_service.say_hello();
        res.send("Hello, Ngyn!");
    }

    #[get(["/bye", "/goodbye"])]
    fn say_goodbye(&self, _req: &NgynRequest, _res: &mut NgynResponse) {
        println!("Goodbye!");
    }
}
