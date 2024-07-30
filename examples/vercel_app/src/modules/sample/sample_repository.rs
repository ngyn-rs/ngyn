use ngyn::prelude::*;

#[injectable]
#[derive(Clone)]
pub struct SampleRepository;

impl SampleRepository {
    pub fn name(&self) -> String {
        "Ngyn".to_string()
    }
}
