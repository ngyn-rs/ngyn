use rustle_core::module;

use super::sample_service::SampleService;

#[module]
pub struct Sample {
    sample_service: SampleService,
}
