use ngyn::{server::VercelApplication, NgynFactory};
use vercel_app::modules::sample::sample_module::SampleModule;
use vercel_runtime::{run, Body, Error, Request, Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let app = NgynFactory::<VercelApplication>::create::<SampleModule>();
    app.handle(req).await
}
