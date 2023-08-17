mod modules;

use modules::sample::sample_module::Sample;
use rustle_core::core::factory::RustleFactory;

fn main() {
    let app = RustleFactory::create::<Sample>();
    println!("Hello, world!");
}
