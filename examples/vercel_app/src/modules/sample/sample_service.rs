use ngyn::prelude::*;

use super::sample_repository::SampleRepository;

#[injectable]
#[derive(Clone)]
pub struct SampleService {
    sample_repository: SampleRepository,
}

impl SampleService {
    pub fn say_hello(&self) {
        println!("Hello, {}!", self.sample_repository.name());
    }
}
