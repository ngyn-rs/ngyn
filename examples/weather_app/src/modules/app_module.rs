use ngyn::prelude::*;

use super::{HomeModule, WeatherModule};
use crate::middlewares::test_middleware::TestMiddleware;

#[module(imports = [HomeModule, WeatherModule], middlewares = [TestMiddleware])]
pub struct AppModule {}
