use ngyn::prelude::*;

use super::weather_gate::WeatherGate;
use super::weather_service::WeatherService;

#[controller(middlewares = [])]
pub struct WeatherController {
    weather_service: WeatherService,
}

#[routes]
impl WeatherController {
    #[check(WeatherGate)]
    #[get("/weather")]
    async fn get_location(&self, _req: &mut NgynRequest, res: &mut NgynResponse) -> String {
        self.weather_service.get_location_weather("London").await
    }
}
