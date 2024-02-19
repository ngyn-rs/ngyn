use ngyn::prelude::*;
use serde::{Serialize, Deserialize};

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
    async fn get_location(&self) -> String {
        self.weather_service.get_location_weather("London").await
    }

    #[post("/weather")]
    #[check(WeatherGate)]
    async fn post_location(&self, weather: WeatherDto) -> String {
        let location = weather.location;
		self.weather_service.get_location_weather("London").await
	}
}
