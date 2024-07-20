use ngyn::shared::{core::NgynEngine, traits::NgynController};
use std::sync::{Arc, Mutex};

pub mod routing;

pub use ngyn_swagger_macros::SwaggerDto;

pub trait SwaggerDto {
    fn to_swagger() -> serde_json::Value;
}

pub trait NgynEngineSwagger: NgynEngine {
    fn use_swagger(&mut self, config: routing::SwaggerConfig) {
        let controller = routing::SwaggerModule::with_config(config);
        let controller = Arc::new(Mutex::new(vec![
            Box::new(controller) as Box<dyn NgynController>
        ]));
        self.load_controller(controller);
    }
}

impl<T: NgynEngine> NgynEngineSwagger for T {}
