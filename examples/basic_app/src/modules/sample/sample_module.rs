use rustle_core::module;

#[module]
struct Sample;

pub trait SampleNew {
    fn new() -> Self;
}

impl SampleNew for Sample {
    fn new() -> Self {
        Sample {
            components: todo!(),
        }
    }
}
