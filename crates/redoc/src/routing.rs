use ngyn::{
    prelude::*,
    shared::traits::{NgynController, NgynModule},
};
use serde_json::{json, Value};

pub trait ReDocValue {
    fn redoc_value(&self) -> Value {
        json!({})
    }
}

#[module]
pub struct ReDocModule;

pub struct ReDocConfig {
    pub spec_url: String,
    pub app_module: Box<dyn NgynModule + Sync>,
    pub title: String,
    pub version: String,
    pub server_url: String,
    pub description: Option<String>,
    pub terms_of_service: Option<String>,
    pub contact: Option<String>,
    pub license: String,
    pub license_url: Option<String>,
}

impl Default for ReDocConfig {
    fn default() -> Self {
        ReDocConfig {
            spec_url: "/docs/openapi.json".to_string(),
            app_module: Box::new(ReDocModule {}),
            title: "API Documentation".to_string(),
            version: "1.0.0".to_string(),
            server_url: "/".to_string(),
            description: None,
            terms_of_service: None,
            contact: None,
            license: "MIT".to_string(),
            license_url: None,
        }
    }
}

#[controller("/docs")]
pub struct ReDocController {
    config: ReDocConfig,
    spec: Value,
}

impl ReDocController {
    pub fn build(&mut self) {
        let app_module = &mut self.config.app_module;
        impl ReDocValue for Box<dyn NgynController> {} // type coercion
        let paths_spec = {
            let controllers = app_module.get_controllers();
            let mut paths = json!({});
            for controller_list in controllers {
                let controller_list = controller_list.lock().unwrap();
                for controller in controller_list.iter() {
                    let routes = controller.routes();
                    let controller_spec = routes
                        .iter()
                        .map(|(path, method, _)| {
                            json!({
                                path.to_string().to_lowercase(): {
                                    method.to_string().to_lowercase(): {
                                        "summary": format!("{} {}", method.to_ascii_uppercase(), path),
                                        "description": "",
                                        "responses": {
                                            "200": {
                                                "description": ""
                                            }
                                        }
                                    }
                                }
                            })
                        })
                        .fold(json!({}), |mut acc, x| {
                            merge(&mut acc, x);
                            acc
                        });
                    merge(&mut paths, controller_spec);
                }
            }
            paths
        };
        self.spec = json!({
            "openapi": "3.0.0",
            "info": {
                "title": self.config.title,
                "version": self.config.version,
                "description": self.config.description,
                "termsOfService": self.config.terms_of_service,
                "contact": self.config.contact,
                "license": {
                    "name": self.config.license,
                    "url": self.config.license_url,
                },
            },
            "servers": [{
                "url": self.config.server_url,
            }],
            "paths": paths_spec,
            "components": {
                "schemas": {}
            }
        });
    }
}

impl ReDocModule {
    pub fn with_config(config: ReDocConfig) -> ReDocController {
        let mut ctrl = ReDocController {
            config,
            spec: json!({}),
        };
        ctrl.build();
        ctrl
    }
}

#[routes]
impl ReDocController {
    #[get("/")]
    async fn index(&self, res: &mut NgynResponse) -> String {
        res.set_header("Content-Type", "text/html");

        let html = include_str!("templates/redoc.html");
        html.replace("% REDOC_SPEC_URL %", &self.config.spec_url)
            .replace("% REDOC_DOC_TITLE %", &self.config.title)
            .replace(
                "% REDOC_DOC_DESCRIPTION %",
                &self.config.description.clone().unwrap_or("".to_string()),
            )
    }

    #[get("/openapi.json")]
    async fn openapi_spec(&mut self) -> String {
        serde_json::to_string_pretty(&self.spec).unwrap_or("{}".to_string())
    }
}

fn merge(a: &mut Value, b: Value) {
    match (a, b) {
        (a @ &mut Value::Object(_), Value::Object(b)) => {
            let a = a.as_object_mut().unwrap();
            for (k, v) in b {
                merge(a.entry(k).or_insert(Value::Null), v);
            }
        }
        (a, b) => *a = b,
    }
}
