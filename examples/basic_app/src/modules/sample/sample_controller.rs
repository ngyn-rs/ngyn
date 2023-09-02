use rustle_core::{controller, get, RustleController, RustleRequest, RustleResponse};

use super::sample_service::SampleService;

#[controller]
pub struct SampleController {
    sample_service: SampleService,
}

impl SampleController {
    #[get("/")]
    fn say_hello(self, _req: RustleRequest, res: RustleResponse) -> RustleResponse {
        self.sample_service.say_hello();
        res
    }

    // #[get(["/bye", "/goodbye"])]
    // pub fn say_goodbye() -> () {
    //     println!("Goodbye!");
    // }
}
