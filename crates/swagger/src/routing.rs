use ngyn::{prelude::*, shared::traits::NgynModule};
use serde_json::{json, Value};

use crate::SwaggerController;

pub struct SwaggerModule;

pub struct SwaggerConfig<AppModule: NgynModule + Clone> {
    pub spec_url: String,
    pub app_module: Box<AppModule>,
    pub title: String,
    pub version: String,
    pub server_url: String,
    pub description: Option<String>,
    pub terms_of_service: Option<String>,
    pub contact: Option<String>,
    pub license: String,
    pub license_url: Option<String>,
}

impl<AppModule: NgynModule + Clone + Default> Default for SwaggerConfig<AppModule> {
    fn default() -> Self {
        SwaggerConfig {
            spec_url: "/docs/openapi.json".to_string(),
            app_module: Box::<AppModule>::default(),
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

#[controller("/docs")]
pub struct SwaggerRoutesController<AppModule: Default + NgynModule + Clone + 'static> {
    config: SwaggerConfig<AppModule>,
    spec: Value,
}

impl<AppModule: Default + NgynModule + Clone + 'static> SwaggerRoutesController<AppModule> {
    pub fn build(&mut self) {
        let app_module = &mut self.config.app_module;
        let (paths_spec, tags, components) = {
            let controllers = app_module.get_controllers();
            let mut paths = json!({});
            let mut components = json!({});
            let mut tags = Vec::new();
            for controller in controllers {
                let swagger_ctrl = controller
                    .as_any()
                    .downcast_ref::<Box<dyn SwaggerController>>();
                if let Some(swagger_ctrl) = swagger_ctrl {
                    let meta = swagger_ctrl.swagger_meta();
                    println!("{:?}", meta.components.len());
                    let components_spec = meta.components.iter().fold(json!({}), |mut acc, x| {
                        println!("{:?}", x);
                        merge(&mut acc, x.clone());
                        acc
                    });
                    merge(&mut components, components_spec);
                }
                let routes = controller.routes();
                let tag_name = controller.prefix().trim_matches('/').to_string();
                tags.push(tag_name.clone());
                let controller_spec = routes
                    .iter()
                    .map(|(path, method, operation_id)| {
                        json!({
                            path.to_lowercase(): {
                                method.to_lowercase(): {
                                    "summary": format!("{} {}", method.to_uppercase(), path),
                                    "description": "",
                                    "operationId": operation_id,
                                    "responses": {
                                        "200": {
                                            "description": ""
                                        }
                                    },
                                    "tags": [tag_name],
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
            (paths, tags, components)
        };
        self.spec = json!({
            "openapi": "3.0.0",
            "info": {
                "title": self.config.title,
                "version": self.config.version,
                "description": self.config.description,
                "termsOfService": self.config.terms_of_service,
                "contact": {
                    "name": self.config.contact,
                },
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
                "schemas": components,
            },
            "tags": tags,
        });
    }
}

impl SwaggerModule {
    pub fn with_config<AppModule: Default + NgynModule + Clone + 'static>(
        config: SwaggerConfig<AppModule>,
    ) -> SwaggerRoutesController<AppModule> {
        let mut ctrl = SwaggerRoutesController {
            config,
            spec: Value::Null,
        };
        ctrl.build();
        ctrl
    }
}

#[routes]
impl<AppModule: Default + NgynModule + Clone + 'static> SwaggerRoutesController<AppModule> {
    #[get("/")]
    async fn index(&self, res: &mut NgynResponse) -> String {
        res.set_header("Content-Type", "text/html");

        let html = include_str!("templates/swagger.html");
        html.replace("% SWAGGER_SPEC_URL %", &self.config.spec_url)
            .replace("% SWAGGER_DOC_TITLE %", &self.config.title)
            .replace(
                "% SWAGGER_DOC_DESCRIPTION %",
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
