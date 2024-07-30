use ngyn::shared::{core::NgynEngine, server::NgynResponse, traits::{NgynController, NgynModule}};
use serde_json::Value;

pub mod routing;

pub use ngyn_swagger_macros::{swagger_route, SwaggerComponent};

pub trait SwaggerComponent {
    fn to_swagger_schema(name: &str) -> serde_json::Value;
    fn to_swagger() -> serde_json::Value;
}

impl SwaggerComponent for bool {
    fn to_swagger_schema(_name: &str) -> serde_json::Value {
        Value::Null
    }
    fn to_swagger() -> serde_json::Value {
        Value::Null
    }
}

impl SwaggerComponent for i32 {
    fn to_swagger_schema(_name: &str) -> serde_json::Value {
        Value::Null
    }
    fn to_swagger() -> serde_json::Value {
        Value::Null
    }
}

impl SwaggerComponent for f32 {
    fn to_swagger_schema(_name: &str) -> serde_json::Value {
        Value::Null
    }
    fn to_swagger() -> serde_json::Value {
        Value::Null
    }
}

impl SwaggerComponent for f64 {
    fn to_swagger_schema(_name: &str) -> serde_json::Value {
        Value::Null
    }
    fn to_swagger() -> serde_json::Value {
        Value::Null
    }
}

impl SwaggerComponent for String {
    fn to_swagger_schema(_name: &str) -> serde_json::Value {
        Value::Null
    }
    fn to_swagger() -> serde_json::Value {
        Value::Null
    }
}

impl SwaggerComponent for Vec<String> {
    fn to_swagger_schema(_name: &str) -> serde_json::Value {
        Value::Null
    }
    fn to_swagger() -> serde_json::Value {
        Value::Null
    }
}

impl SwaggerComponent for Value {
    fn to_swagger_schema(_name: &str) -> serde_json::Value {
        Value::Null
    }
    fn to_swagger() -> serde_json::Value {
        Value::Null
    }
}

impl SwaggerComponent for NgynResponse {
    fn to_swagger_schema(_name: &str) -> serde_json::Value {
        Value::Null
    }
    fn to_swagger() -> serde_json::Value {
        Value::Null
    }
}

pub struct SwaggerMeta {
    pub components: Vec<serde_json::Value>,
    pub operations: Vec<(String, Vec<serde_json::Value>)>,
    pub response: serde_json::Value,
}

pub trait SwaggerController: NgynController {
    fn swagger_meta(&self) -> SwaggerMeta {
        SwaggerMeta {
            components: Vec::new(),
            operations: Vec::new(),
            response: Value::Null,
        }
    }
}

pub trait NgynEngineSwagger: NgynEngine {
    fn use_swagger<AppModule: Default + NgynModule + Clone + 'static>(&mut self, config: routing::SwaggerConfig<AppModule>) {
        let controller = routing::SwaggerModule::with_config(config);
        self.load_controller(controller);
    }
}

impl<T: NgynEngine> NgynEngineSwagger for T {}
