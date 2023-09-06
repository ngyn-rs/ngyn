use ngyn::injectable;

use super::sample_repository::SampleRepository;

#[injectable]
pub struct SampleService {
    sample_repository: SampleRepository,
}

impl SampleService {
    #[allow(dead_code)]
    pub fn say_hello(&self) {
        println!("Hello, {}!", self.sample_repository.name());
    }
}
