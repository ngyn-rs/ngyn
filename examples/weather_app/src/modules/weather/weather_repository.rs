use ngyn::injectable;
use std::{collections::HashMap, env, error::Error};

#[injectable]
pub struct WeatherRepository;

impl WeatherRepository {
    fn build_url(&self, api_type: &str, location: &str) -> String {
        let api_key = env::var("WEATHER_API_KEY").unwrap();
        format!(
            "http://api.weatherapi.com/v1/{}.json?key={}&q={}",
            api_type, api_key, location
        )
    }

    async fn send_request(&self, url: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let response = reqwest::get(url)
            .await?
            .json::<HashMap<String, String>>()
            .await?;

        Ok(response)
    }

    pub async fn get_location_current_weather(&self, location: &str) -> HashMap<String, String> {
        println!("Getting weather for {}", location);
        let url = self.build_url("current", location);
        println!("Sending request to {}", url);
        let resp = self.send_request(&url).await.unwrap();
        resp
    }
}
