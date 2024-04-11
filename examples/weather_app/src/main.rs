mod middlewares;
mod modules;

use dotenv::dotenv;
use modules::AppModule;
use ngyn::{
    platforms::HyperApplication,
    prelude::*,
};

#[ngyn::macros::main]
async fn main() {
    dotenv().ok();
    let app = NgynFactory::<HyperApplication>::create::<AppModule>();

    println!("Starting server at http://127.0.0.1:8080");

    let _ = app.listen("127.0.0.1:8080").await;
}
