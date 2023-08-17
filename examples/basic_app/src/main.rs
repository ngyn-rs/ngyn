mod modules;

use modules::sample::sample_module::Sample;
use rustle_core::RustleFactory;

fn main() {
    let app = RustleFactory::create::<Sample>();
    println!("Hello, world!");
}
