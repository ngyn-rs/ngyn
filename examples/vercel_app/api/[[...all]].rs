use ngyn_swagger::{NgynEngineSwagger, SwaggerConfig};
use ngyn_vercel::VercelApplication;
use vercel_runtime::{run, Body, Error, Request, Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let mut app = VercelApplication::default();
    app.use_swagger(SwaggerConfig {
        spec_url: "/openapi.json".to_string(),
        ..Default::default()
    });
    app.handle(req).await
}
