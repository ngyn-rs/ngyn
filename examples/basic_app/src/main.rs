mod modules;

use modules::sample::sample_module::Sample;
use rustle_core::RustleFactory;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let app = RustleFactory::create::<Sample>();

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
