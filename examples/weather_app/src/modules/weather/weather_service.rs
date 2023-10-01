use ngyn::injectable;
use std::collections::HashMap;

use super::weather_repository::WeatherRepository;

#[injectable]
pub struct WeatherService {
    weather_repository: WeatherRepository,
}

impl WeatherService {
    pub async fn get_location_weather(&self, location: &str) -> HashMap<String, String> {
        println!("Getting weather for {}", location);
        self.weather_repository
            .get_location_current_weather(location)
            .await
    }
}
