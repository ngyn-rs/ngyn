use ngyn::{prelude::*, shared::traits::NgynModule};
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
}

impl ReDocModule {
    pub fn with_config(config: ReDocConfig) -> ReDocController {
        ReDocController { config }
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
    async fn openapi_spec(&self) -> String {
        let app_module = &self.config.app_module;
        impl ReDocValue for Box<dyn NgynModule + Sync> {} // type coercion
        let paths_spec = app_module.redoc_value();
        let full_spec = json!({
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
                "url": "/api/v3"
            }],
            "paths": paths_spec,
            "components": {
                "schemas": {}
            }
        });
        serde_json::to_string_pretty(&full_spec).unwrap_or("{}".to_string())
    }
}
