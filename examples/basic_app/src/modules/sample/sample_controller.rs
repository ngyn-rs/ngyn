use rustle_core::{controller, get};

use super::sample_service::SampleService;

#[controller]
pub struct SampleController {
    sample_service: SampleService,
}

impl SampleController {
    #[get("/")]
    pub fn say_hello(self) -> () {
        self.sample_service.say_hello();
    }

    #[get(["/bye", "/goodbye"])]
    pub fn say_goodbye() -> () {
        println!("Goodbye!");
    }
}
