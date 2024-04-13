use ngyn::prelude::*;

use super::sample_service::SampleService;

#[controller]
pub struct SampleController {
    sample_service: SampleService,
}

#[routes]
impl SampleController {
    #[get("/hello")]
    fn say_hello(&self) {
        self.sample_service.say_hello();
        "Hello, Ngyn from Vercel!".to_string();
    }

    #[get(["/bye", "/goodbye"])]
    fn say_goodbye(&self) {
        "Goodbye from Vercel!".to_string();
    }
}
