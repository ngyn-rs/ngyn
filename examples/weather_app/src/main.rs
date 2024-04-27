mod middlewares;
mod modules;

use dotenv::dotenv;
use modules::AppModule;
use ngyn::{platforms::HyperApplication, prelude::*};

use crate::middlewares::notfound_middleware::NotFoundMiddleware;

#[ngyn::macros::main]
async fn main() {
    dotenv().ok();
    let mut app = NgynFactory::<HyperApplication>::create::<AppModule>();

    println!("Starting server at http://127.0.0.1:8080");

    app.use_middleware(NotFoundMiddleware::new());

    let _ = app.listen("127.0.0.1:8080").await;
}
