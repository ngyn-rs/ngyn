use ngyn::{controller, get, NgynRequest, NgynResponse};

use super::weather_service::WeatherService;

#[controller("get_location", middlewares = [])]
pub struct WeatherController {
    weather_service: WeatherService,
}

impl WeatherController {
    #[get("/weather")]
    async fn get_location(self, _req: NgynRequest, res: NgynResponse) -> NgynResponse {
        let weather = self.weather_service.get_location_weather("London").await;
        res.body(weather.get("current").unwrap())
    }
}
