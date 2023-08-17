use rustle_core::injectable;

#[injectable]
pub struct SampleService {
    #[allow(dead_code)]
    pub name: String,
}

impl Default for SampleService {
    fn default() -> Self {
        SampleService {
            name: "default".to_string(),
        }
    }
}
