use ngyn::prelude::*;

#[module(controllers = [ReDocController])]
pub struct ReDocModule;

#[controller]
pub struct ReDocController;

#[routes]
impl ReDocController {
    #[get("/docs")]
    async fn index(&self, res: &mut NgynResponse) -> String {
        res.set_header("Content-Type", "text/html");

        let html = include_str!("templates/redoc.html");
        html.replace("% REDOC_SPEC_URL %", "")
    }

    #[get("/docs/openapi.json")]
    async fn openapi_spec(&self) {}
}
