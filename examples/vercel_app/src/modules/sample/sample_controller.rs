use ngyn::prelude::*;
use ngyn_swagger::SwaggerDto;

use super::sample_service::SampleService;

#[derive(SwaggerDto)]
pub struct SampleDto {
    pub name: String,
    pub age: i32,
}

#[controller]
pub struct SampleController {
    sample_service: SampleService,
}

#[routes]
impl SampleController {
    #[get("/hello")]
    fn say_hello(&self) -> String {
        self.sample_service.say_hello();
        SampleDto::to_swagger().to_string()
    }

    #[get(["/bye", "/goodbye"])]
    fn say_goodbye(&self) {
        "Goodbye from Vercel!".to_string();
    }
}
