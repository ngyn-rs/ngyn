mod modules;

use modules::sample::sample_module::SampleModule;
use rustle_core::{Result, RustleFactory};

#[async_std::main]
async fn main() -> Result<()> {
    let app = RustleFactory::create::<SampleModule>();

    println!("Starting server at http://127.0.0.1:8080");

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
