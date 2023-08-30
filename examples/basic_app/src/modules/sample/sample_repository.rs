use rustle_core::injectable;

#[injectable]
pub struct SampleRepository;

impl SampleRepository {
    pub fn name(&self) -> String {
        "Rustle".to_string()
    }
}
