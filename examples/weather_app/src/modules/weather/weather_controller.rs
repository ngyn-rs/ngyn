use ngyn::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use validator::Validate;

use super::weather_gate::WeatherGate;
use super::weather_service::WeatherService;
use crate::middlewares::test_middleware::TestMiddleware;

#[derive(Dto, Validate, Serialize, Deserialize)]
pub struct WeatherDto {
    pub location: String,
    pub temperature: f32,
    pub humidity: f32,
}

#[derive(Param)]
pub struct GetWeatherParams {
    pub location: String,
    // pub city: String,
}

#[controller(prefix="/weather", middlewares=[TestMiddleware, TestMiddleware])]
pub struct WeatherController {
    #[inject]
    weather_service: WeatherService,
}

#[routes]
impl WeatherController {
    #[get("/<location>/<city>")]
    #[check(WeatherGate)]
    async fn get_location(&self, params: GetWeatherParams) -> Result<String, Value> {
        println!("{:?}", "Getting location weather");
        if !params.location.is_empty() {
            match self.weather_service.get_weather(&params.location).await {
                Ok(r) => Ok(r),
                Err(e) => Err(json!({ "status": 501, "message": e.to_string() })),
            }
        } else {
            Err(json!({ "status": 401, "message": "please specify location param" }))
        }
    }

    #[post("/")]
    async fn post_location(&self, weather: WeatherDto) -> Result<String, Value> {
        let location = weather.location;
        match self.weather_service.get_weather(&location).await {
            Ok(r) => Ok(r),
            Err(e) => Err(json!({ "status": 501, "message": e.to_string() })),
        }
    }
}
