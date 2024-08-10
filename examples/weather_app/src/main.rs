mod middlewares;
mod modules;
mod shared;

use std::sync::Arc;

use dotenv::dotenv;
use modules::AppModule;
use ngyn::prelude::*;
use ngyn_shuttle::{ShuttleApplication, ShuttleNgyn};

use crate::middlewares::notfound_middleware::NotFoundMiddleware;

#[route("GET", "/")]
pub async fn fn_route() -> String {
    "Hello, World!".to_string()
}

#[shuttle_runtime::main]
async fn main() -> ShuttleNgyn {
    dotenv().ok();
    let mut app = NgynFactory::<ShuttleApplication>::create::<AppModule>();

    app.use_middleware(NotFoundMiddleware::new());
    app.use_interpreter(shared::ResponseInterpreter {});
    app.load_controller(Arc::new(Box::new(fn_route.into())));

    Ok(app.into())
}
