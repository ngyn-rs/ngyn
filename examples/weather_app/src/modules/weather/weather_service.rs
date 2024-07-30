use ngyn::prelude::*;

use super::weather_repository::WeatherRepository;

#[injectable]
#[derive(Clone)]
pub struct WeatherService {
    weather_repository: WeatherRepository,
}

impl WeatherService {
    pub async fn get_location_weather(&self, location: &str) -> String {
        println!("Getting weather for {}", location);
        self.weather_repository
            .get_location_current_weather(location)
            .await
    }
}
