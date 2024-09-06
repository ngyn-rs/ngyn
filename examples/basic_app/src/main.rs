mod modules;

use modules::sample::sample_module::SampleModule;
use ngyn::prelude::*;

#[tokio::main]
async fn main() {
    let mut app = NgynFactory::<HyperApplication>::create::<SampleModule>();

    app.get(
        "/author",
        handler(|_cx: &mut NgynContext| "Ngyn is created by @elcharitas."),
    );

    println!("Starting server at http://127.0.0.1:8080");

    let _ = app.listen("0.0.0.0:8080").await;
}
