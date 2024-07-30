use ngyn::prelude::*;
use std::env;

#[injectable]
#[derive(Clone)]
pub struct WeatherRepository;

impl WeatherRepository {
    fn build_url(&self, api_type: &str, location: &str) -> String {
        let api_key = env::var("WEATHER_API_KEY").unwrap();
        format!(
            "https://api.weatherapi.com/v1/{}.json?key={}&q={}",
            api_type, api_key, location
        )
    }

    async fn send_request(&self, url: &str) -> Result<String, ureq::Error> {
        let response = ureq::get(url).call()?.into_string()?;
        Ok(response)
    }

    pub async fn get_location_current_weather(&self, location: &str) -> String {
        println!("Getting weather for {}", location);
        let url = self.build_url("current", location);
        println!("Sending request to {}", url);
        self.send_request(&url).await.unwrap()
    }
}
