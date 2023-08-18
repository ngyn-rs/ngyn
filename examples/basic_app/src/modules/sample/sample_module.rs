use rustle_core::module;

use super::sample_service::SampleService;

#[module]
pub struct SampleModule {
    controllers: Vec<Box<SampleService>>,
}
