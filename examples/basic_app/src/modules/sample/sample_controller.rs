use rustle_core::{controller, get, RustleRequest, RustleResponse};

use super::sample_service::SampleService;

#[controller("say_hello, say_goodbye")]
pub struct SampleController {
    sample_service: SampleService,
}

impl SampleController {
    #[get("/")]
    fn say_hello(self, _req: RustleRequest, res: RustleResponse) {
        self.sample_service.say_hello();
        res
    }

    #[get(["/bye", "/goodbye"])]
    fn say_goodbye(self, _req: RustleRequest, res: RustleResponse) {
        println!("Goodbye!");
        res
    }
}
