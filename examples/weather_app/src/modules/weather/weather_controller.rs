use ngyn::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::weather_gate::WeatherGate;
use super::weather_service::WeatherService;

#[derive(Dto, Serialize, Deserialize, Validate)]
pub struct WeatherDto {
    pub location: String,
    pub temperature: f32,
    pub humidity: f32,
}

#[controller("/weather")]
pub struct WeatherController {
    weather_service: WeatherService,
}

#[routes]
impl WeatherController {
    #[get("/<location>/<city>")]
    #[check(WeatherGate)]
    async fn get_location(&self, params: Param) -> String {
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
