use ngyn::prelude::*;
use ngyn_swagger::SwaggerComponent;
use serde::{Deserialize, Serialize};

use super::sample_service::SampleService;

#[derive(Dto, Serialize, Deserialize, SwaggerComponent)]
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
    fn say_hello(&self) -> SampleDto {
        self.sample_service.say_hello();
        SampleDto {
            name: "Vercel".to_string(),
            age: 1,
        }
    }

    #[get(["/bye", "/goodbye"])]
    fn say_goodbye(&self, _body: SampleDto) -> String {
        "Goodbye from Vercel!".to_string()
    }
}
