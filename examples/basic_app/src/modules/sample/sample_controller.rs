use rustle_core::controller;

use super::sample_service::SampleService;

#[controller]
pub struct SampleController {
    sample_service: SampleService,
}

impl SampleController {
    #[allow(dead_code)]
    pub fn say_hello(&self) {
        self.sample_service.say_hello();
    }
}
