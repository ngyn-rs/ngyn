use ngyn::{http::StatusCode, prelude::*, shared::server::Transformer};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::middlewares::test_middleware::TestMiddleware;

pub struct WeatherGate;

impl NgynGate for WeatherGate {
    async fn can_activate(cx: &mut NgynContext) -> bool {
        let query = Query::transform(cx);
        if query.get("location").is_some() {
            return true;
        }
        *cx.response().status_mut() = StatusCode::BAD_REQUEST;
        *cx.response().body_mut() = "Bad Request: location query parameter is required".into();
        // prevent activation of the next components
        false
    }
}

#[injectable]
pub struct WeatherRepository;

impl WeatherRepository {
    fn build_url(&self, api_type: &str, location: &str) -> String {
        let api_key = std::env::var("WEATHER_API_KEY").unwrap();
        format!(
            "https://api.weatherapi.com/v1/{}.json?key={}&q={}",
            api_type, api_key, location
        )
    }

    async fn send_request(&self, url: &str) -> Result<String, ureq::Error> {
        let response = ureq::get(url).call()?.into_string()?;
        Ok(response)
    }

    pub async fn get_current_weather(&self, location: &str) -> Result<String, ureq::Error> {
        println!("Getting weather for {}", location);
        let url = self.build_url("current", location);
        println!("Sending request to {}", url);
        self.send_request(&url).await
    }
}

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

#[derive(Dto, Validate, Serialize, Deserialize)]
pub struct WeatherDto {
    pub location: String,
    pub temperature: f32,
    pub humidity: f32,
}

#[derive(Param)]
pub struct GetWeatherParams {
    pub location: String,
    // pub city: String,
}

#[handler(middlewares=[TestMiddleware])]
pub async fn get_location(
    params: GetWeatherParams,
    weather_service: WeatherService,
) -> Result<String, Value> {
    println!("{:?}", "Getting location weather");
    if !params.location.is_empty() {
        match weather_service.get_weather(&params.location).await {
            Ok(r) => Ok(r),
            Err(e) => Err(json!({ "status": 501, "message": e.to_string() })),
        }
    } else {
        Err(json!({ "status": 401, "message": "please specify location param" }))
    }
}

#[handler]
pub async fn post_location(
    weather: WeatherDto,
    weather_service: WeatherService,
) -> Result<String, Value> {
    let location = weather.location;
    match weather_service.get_weather(&location).await {
        Ok(r) => Ok(r),
        Err(e) => Err(json!({ "status": 501, "message": e.to_string() })),
    }
}
