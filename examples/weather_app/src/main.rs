mod middlewares;
mod modules;
mod shared;

use dotenv::dotenv;
use modules::{get_location, post_location};
use ngyn::prelude::*;
use ngyn_shuttle::{ShuttleApplication, ShuttleNgyn};

use crate::middlewares::notfound_middleware::NotFoundMiddleware;

#[shuttle_runtime::main]
async fn main() -> ShuttleNgyn {
    dotenv().ok();
    let mut app = ShuttleApplication::default();

    app.get("/<location>/<city>", async_wrap(get_location));
    app.post("/", async_wrap(post_location));

    app.use_middleware(NotFoundMiddleware {});
    app.use_interpreter(shared::ResponseInterpreter {});

    Ok(app.into())
}
