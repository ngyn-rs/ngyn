use rustle_core::{controller, module};

use super::sample_controller::SampleController;

#[controller]
pub struct SampleController2 {}

#[module([SampleController, SampleController2])]
pub struct SampleModule {}
