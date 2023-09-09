use ngyn::module;

use super::weather_controller::WeatherController;

#[module(controllers = [WeatherController])]
pub struct WeatherModule {}
