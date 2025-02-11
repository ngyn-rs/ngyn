mod middlewares;
mod weather;

use dotenv::dotenv;
use ngyn::prelude::HyperConfig;
use ngyn::prelude::*;
use ngyn_shuttle::{ShuttleApplication, ShuttleNgyn};
use weather::{get_location, post_location};

use crate::middlewares::notfound_middleware::NotFoundMiddleware;

#[shuttle_runtime::main]
async fn main() -> ShuttleNgyn {
    dotenv().ok();
    let mut app = ShuttleApplication::with_config(HyperConfig::default());

    app.get("/{location}/{city}", async_wrap(get_location));
    app.any("/", async_wrap(post_location));

    app.use_middleware(NotFoundMiddleware {});

    Ok(app.into())
}
