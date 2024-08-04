use super::sample_controller::SampleController;

#[ngyn::macros::module(controllers = [SampleController])]
#[derive(Default, Clone)]
pub struct SampleModule {}
