mod middlewares;
mod modules;

use dotenv::dotenv;
use modules::AppModule;
use ngyn::prelude::*;
use ngyn_shuttle::{ShuttleApplication, ShuttleNgyn};

use crate::middlewares::notfound_middleware::NotFoundMiddleware;

#[shuttle_runtime::main]
async fn main() -> ShuttleNgyn {
    dotenv().ok();
    let mut app = NgynFactory::<ShuttleApplication>::create::<AppModule>();
    app.use_middleware(NotFoundMiddleware::new());

    Ok(app.into())
}
