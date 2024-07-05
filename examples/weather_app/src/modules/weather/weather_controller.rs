use ngyn::prelude::*;
use serde::{Deserialize, Serialize};
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

#[controller(prefix="/weather", middlewares=[TestMiddleware])]
pub struct WeatherController {
    weather_service: WeatherService,
}

#[routes]
impl WeatherController {
    #[get("/<location>/<city>")]
    #[check(WeatherGate)]
    async fn get_location(&self, params: Query) -> String {
        self.weather_service
            .get_location_weather(params.get("location").unwrap().as_str())
            .await
    }

    #[post("/")]
    async fn post_location(&self, weather: WeatherDto) -> String {
        let location = weather.location;
        self.weather_service.get_location_weather(&location).await
    }
}
