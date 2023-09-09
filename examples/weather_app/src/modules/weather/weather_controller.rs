use ngyn::{controller, get, NgynRequest, NgynResponse};

use super::weather_service::WeatherService;

#[controller("get_location")]
pub struct WeatherController {
    weather_service: WeatherService,
}

impl WeatherController {
    #[get("/weather")]
    fn get_location(self, _req: NgynRequest, res: NgynResponse) -> NgynResponse {
        res.body(&self.weather_service.get_location_weather("London"))
    }
}
