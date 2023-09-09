use ngyn::module;

use super::{HomeModule, WeatherModule};

#[module(imports = [HomeModule, WeatherModule])]
pub struct AppModule {}
