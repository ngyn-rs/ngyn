use ngyn::injectable;
use std::{
    env,
    error::Error,
    io::{Read, Write},
    net::TcpStream,
};

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

    fn send_request(&self, url: &str) -> Result<String, Box<dyn Error>> {
        let mut stream = TcpStream::connect("api.weatherapi.com:80")?;
        let request = format!("GET {} HTTP/1.1\r\nHost: api.weatherapi.com\r\n\r\n", url);

        println!("Sending request: {}", request);

        stream.write_all(request.as_bytes())?;

        let mut response = String::new();
        stream.read_to_string(&mut response)?;

        Ok(response)
    }

    pub fn get_location_current_weather(&self, location: &str) -> String {
        println!("Getting weather for {}", location);
        let url = self.build_url("current", location);
        println!("Sending request to {}", url);
        let resp = self.send_request(&url).unwrap();
        resp
    }
}
