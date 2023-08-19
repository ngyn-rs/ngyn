use rustle_core::injectable;

#[injectable]
pub struct SampleRepository;

impl SampleRepository {
    #[allow(dead_code)]
    pub fn name(&self) -> String {
        "SampleRepository".to_string()
    }
}

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
