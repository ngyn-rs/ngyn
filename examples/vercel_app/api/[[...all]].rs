use ngyn::factory::NgynFactory;
use ngyn_swagger::NgynEngineSwagger;
use ngyn_vercel::VercelApplication;
use vercel_app::modules::sample::sample_module::SampleModule;
use vercel_runtime::{run, Body, Error, Request, Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let mut app = NgynFactory::<VercelApplication>::create::<SampleModule>();
    app.use_swagger::<SampleModule>(Default::default());
    app.handle(req).await
}
