use rustle_core::injectable;

#[injectable]
pub struct SampleRepository;

impl SampleRepository {
    #[allow(dead_code)]
    pub fn name(&self) -> String {
        "Rustle".to_string()
    }
}
