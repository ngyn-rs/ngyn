use ngyn::http::StatusCode;
use ngyn::prelude::*;
use serde::{Deserialize, Serialize};

use super::weather_gate::WeatherGate;
use super::weather_service::WeatherService;

#[derive(Dto, Serialize, Deserialize)]
pub struct WeatherDto {
    pub location: String,
    pub temperature: f32,
    pub humidity: f32,
}

#[controller(middlewares = [])]
pub struct WeatherController {
    weather_service: WeatherService,
}

#[routes]
impl WeatherController {
    #[get("/weather")]
    #[check(WeatherGate)]
    async fn get_location(&self, query: Query) -> String {
        let location = query.get("location").unwrap_or_default();
        if location.is_empty() {
            eject!(res, StatusCode::BAD_REQUEST, "Location is required");
        }
        self.weather_service.get_location_weather(&location).await
    }

    #[post("/weather")]
    #[check(WeatherGate)]
    async fn post_location(&self, weather: WeatherDto) -> String {
        let location = weather.location;
        self.weather_service.get_location_weather(&location).await
    }
}
