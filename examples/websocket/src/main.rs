use ngyn::prelude::*;
use ngyn_websocket::WebsocketApplication;

#[tokio::main]
async fn main() {
    let mut app = WebsocketApplication::default();

    app.any("/", handler(|_| "Hello"));

    println!("Starting server at ws://127.0.0.1:8080");

    let _ = app.listen("0.0.0.0:8080");
}
