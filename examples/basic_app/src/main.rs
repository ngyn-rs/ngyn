use std::path::PathBuf;

use ngyn::prelude::*;

#[tokio::main]
async fn main() {
    let mut app = HyperApplication::default();

    app.get(
        "/author",
        handler(|_| Ok::<&str, ()>("Ngyn is created by @elcharitas.")),
    );

    let _ = app.use_static(PathBuf::from("static"));

    println!("Starting server at http://127.0.0.1:8080");

    let _ = app.listen("0.0.0.0:8080").await;
}
