use ngyn::prelude::*;

#[injectable]
pub struct SampleRepository;

impl SampleRepository {
    pub fn name(&self) -> String {
        "Ngyn".to_string()
    }
}
