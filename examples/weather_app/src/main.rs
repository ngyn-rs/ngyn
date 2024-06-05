mod middlewares;
mod modules;

use dotenv::dotenv;
use modules::AppModule;
use ngyn::prelude::*;
use ngyn_hyper::HyperApplication;
use shuttle_ngyn::ShuttleNgyn;

use crate::middlewares::notfound_middleware::NotFoundMiddleware;

#[shuttle_runtime::main]
async fn main() -> ShuttleNgyn {
    dotenv().ok();
    let mut app = NgynFactory::<HyperApplication>::create::<AppModule>();
    app.use_middleware(NotFoundMiddleware::new());

    Ok(app.into())
}
