use ngyn::shared::core::engine::RouteInstance;
use ngyn::shared::{
    core::{
        engine::{NgynHttpEngine, PlatformData},
        handler::handler,
    },
    server::{NgynContext, NgynResponse},
};
use serde_json::{json, Value};

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

pub struct SwaggerConfig {
    pub spec_url: String,
    pub title: String,
    pub version: String,
    pub server_url: String,
    pub description: Option<String>,
    pub terms_of_service: Option<String>,
    pub contact: Option<String>,
    pub license: String,
    pub license_url: Option<String>,
}

impl Default for SwaggerConfig {
    fn default() -> Self {
        SwaggerConfig {
            spec_url: "/docs/openapi.json".to_string(),
            title: "API Documentation".to_string(),
            version: "1.0.0".to_string(),
            server_url: '/'.to_string(),
            description: None,
            terms_of_service: Some("".to_string()),
            contact: None,
            license: "MIT".to_string(),
            license_url: None,
        }
    }
}

pub fn build_specs_with_config(config: &SwaggerConfig, _platform: &mut PlatformData) -> Value {
    json!({
        "openapi": "3.0.0",
        "info": {
            "title": config.title,
            "version": config.version,
            "description": config.description,
            "termsOfService": config.terms_of_service,
            "contact": {
                "name": config.contact,
            },
            "license": {
                "name": config.license,
                "url": config.license_url,
            },
        },
        "servers": [{
            "url": config.server_url,
        }],
        // "paths": paths_spec,
        // "components": {
        //     "schemas": components,
        // },
        // "tags": tags,
    })
}

pub trait NgynEngineSwagger: NgynHttpEngine {
    fn use_swagger(&mut self, config: SwaggerConfig) {
        let template = include_str!("templates/swagger.html");
        let docs_body = template
            .replace("% SWAGGER_SPEC_URL %", &config.spec_url)
            .replace("% SWAGGER_DOC_TITLE %", &config.title)
            .replace(
                "% SWAGGER_DOC_DESCRIPTION %",
                &config.description.clone().unwrap_or("".to_string()),
            );

        let spec = build_specs_with_config(&config, self.data_mut());
        let spec_json = serde_json::to_string(&spec).unwrap_or("{}".to_string());

        self.get(&config.spec_url, handler(move |_c| spec_json.clone()));
        self.get(
            "/docs",
            handler(move |_c: &mut NgynContext| docs_body.clone()),
        );
    }
}

impl<T: NgynHttpEngine> NgynEngineSwagger for T {}

// fn merge(a: &mut Value, b: Value) {
//     match (a, b) {
//         (a @ &mut Value::Object(_), Value::Object(b)) => {
//             let a = a.as_object_mut().unwrap();
//             for (k, v) in b {
//                 merge(a.entry(k).or_insert(Value::Null), v);
//             }
//         }
//         (a, b) => *a = b,
//     }
// }
