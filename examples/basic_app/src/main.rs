mod modules;

use modules::sample::sample_module::SampleModule;
use ngyn::{NgynFactory, Result};

#[async_std::main]
async fn main() -> Result<()> {
    let mut app = NgynFactory::create::<SampleModule>();

    app.get("/author", |_req, res| {
        res.body("Ngyn is created by @elcharitas.")
    });

    println!("Starting server at http://127.0.0.1:8080");

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
