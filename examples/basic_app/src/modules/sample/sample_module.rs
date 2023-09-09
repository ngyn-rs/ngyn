use ngyn::module;

use super::sample_controller::SampleController;

#[module(controllers = [SampleController])]
pub struct SampleModule {}
