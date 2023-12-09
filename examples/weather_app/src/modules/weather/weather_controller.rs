use ngyn::{check, controller, get, NgynBody, NgynRequest, NgynResponse};

use super::weather_gate::WeatherGate;
use super::weather_service::WeatherService;

#[controller("get_location", middlewares = [])]
pub struct WeatherController {
    weather_service: WeatherService,
}

impl WeatherController {
    #[check(WeatherGate)]
    #[get("/weather")]
    async fn get_location(&self, _req: &NgynRequest, res: &mut NgynResponse) -> NgynBody {
        let weather = self.weather_service.get_location_weather("London").await;
        weather.into()
    }
}
