use ngyn_core::{controller, get, NgynRequest, NgynResponse};

use super::sample_service::SampleService;

#[controller("say_hello, say_goodbye")]
pub struct SampleController {
    sample_service: SampleService,
}

impl SampleController {
    #[get("/")]
    fn say_hello(self, _req: NgynRequest, res: NgynResponse) -> NgynResponse {
        self.sample_service.say_hello();
        res.body("Hello, Ngyn!")
    }

    #[get(["/bye", "/goodbye"])]
    fn say_goodbye(self, _req: NgynRequest, res: NgynResponse) -> NgynResponse {
        println!("Goodbye!");
        res
    }
}
