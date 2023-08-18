use rustle_core::injectable;

#[injectable]
pub struct SampleService {
    pub name: String,
}

impl SampleService {
    #[allow(dead_code)]
    pub fn say_hello(&self) {
        println!("Hello, {}!", self.name);
    }
}
