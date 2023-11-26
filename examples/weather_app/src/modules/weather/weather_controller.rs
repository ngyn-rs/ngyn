use ngyn::{check, controller, get, NgynRequest, NgynResponse};

use super::weather_gate::WeatherGate;
use super::weather_service::WeatherService;

#[controller("get_location", middlewares = [])]
pub struct WeatherController {
    weather_service: WeatherService,
}

impl WeatherController {
    #[get("/weather")]
    #[check(WeatherGate)]
    async fn get_location(&self, _req: &NgynRequest, res: &mut NgynResponse) -> NgynResponse {
        let weather = self.weather_service.get_location_weather("London").await;
        res.send(weather.as_str()).clone()
    }
}
