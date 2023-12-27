use ngyn::macros::*;

use super::weather_controller::WeatherController;

#[module(controllers = [WeatherController])]
pub struct WeatherModule {}
