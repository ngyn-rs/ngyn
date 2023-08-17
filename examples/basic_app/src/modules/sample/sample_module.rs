use rustle_core::module;

#[module]
struct Sample;

pub trait SampleModule {
    fn new() -> Self;
}

impl SampleModule for Sample {
    fn new() -> Self {
        Sample { components: vec![] }
    }
}
