mod modules;

use modules::sample::sample_module::SampleModule;
use ngyn::{
    platforms::{NgynApplication, Result},
    prelude::*,
};

#[ngyn::macros::main]
async fn main() -> Result<()> {
    let mut app = NgynFactory::<NgynApplication>::create::<SampleModule>();

    app.get(
        "/author",
        |_req: &mut NgynRequest, res: &mut NgynResponse| {
            res.send("Ngyn is created by @elcharitas.");
        },
    );

    println!("Starting server at http://127.0.0.1:8080");

    app.listen("0.0.0.0:8080").await?;

    Ok(())
}
