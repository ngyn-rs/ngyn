use rustle_core::module;

#[module]
pub struct Sample {
    field1: i32,
}

pub trait SampleModule {
    fn new() -> Self;
}

impl SampleModule for Sample {
    fn new() -> Self {
        Sample {
            components: vec![],
            field1: 0,
        }
    }
}
