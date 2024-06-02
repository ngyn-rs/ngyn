use ngyn::prelude::*;

use super::{HomeModule, WeatherModule};

#[module(imports = [HomeModule, WeatherModule])]
pub struct AppModule {}
