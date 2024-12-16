mod routes;

use ngyn::prelude::*;
use routes::echo_route::echo_route;

#[tokio::main]
async fn main() {
    let mut app = HyperApplication::default();

    app.get("/", handler(echo_route));

    println!("Starting server at http://127.0.0.1:8080");
    let _ = app.listen("0.0.0.0:8080").await;
}
