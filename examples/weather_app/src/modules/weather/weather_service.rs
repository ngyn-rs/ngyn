use ngyn::prelude::*;

use super::weather_repository::WeatherRepository;

#[injectable]
pub struct WeatherService {
    weather_repository: WeatherRepository,
}

impl WeatherService {
    pub async fn get_weather(&self, location: &str) -> Result<String, ureq::Error> {
        println!("Getting weather for {}", location);
        self.weather_repository.get_current_weather(location).await
    }
}
