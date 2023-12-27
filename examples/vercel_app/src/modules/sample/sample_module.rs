use super::sample_controller::SampleController;

#[ngyn::macros::module(controllers = [SampleController])]
pub struct SampleModule {}
