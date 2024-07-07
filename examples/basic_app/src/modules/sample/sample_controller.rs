use ngyn::prelude::*;

use super::sample_service::SampleService;

#[controller]
pub struct SampleController {
    sample_service: SampleService,
}

#[routes]
impl SampleController {
    #[get(["/", "/hello"])]
    fn say_hello(&self) -> &str {
        self.sample_service.say_hello();
        "Hello from Ngyn!"
    }

    #[get(["/bye", "/goodbye"])]
    fn say_goodbye(&self) -> &str {
        "Goodbye from Ngyn!"
    }
}
