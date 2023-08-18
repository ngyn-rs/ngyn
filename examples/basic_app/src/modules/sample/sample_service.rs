use rustle_core::{injectable, RustleInjectable};

#[injectable]
pub struct SampleService {
    pub name: String,
}

impl RustleInjectable for SampleService {
    fn new() -> Self {
        SampleService {
            name: "default".to_string(),
        }
    }
}

impl SampleService {
    #[allow(dead_code)]
    pub fn say_hello(&self) {
        println!("Hello, {}!", self.name);
    }
}
