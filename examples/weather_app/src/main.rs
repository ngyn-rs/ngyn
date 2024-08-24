mod middlewares;
mod modules;
mod shared;

use dotenv::dotenv;
use modules::AppModule;
use ngyn::prelude::*;
use ngyn_shuttle::{ShuttleApplication, ShuttleNgyn};

use crate::middlewares::notfound_middleware::NotFoundMiddleware;

#[shuttle_runtime::main]
async fn main() -> ShuttleNgyn {
    dotenv().ok();
    let mut app: ShuttleApplication = NgynFactory::create::<AppModule>();

    app.use_middleware(NotFoundMiddleware::new());
    app.use_interpreter(shared::ResponseInterpreter {});

    Ok(app.into())
}
