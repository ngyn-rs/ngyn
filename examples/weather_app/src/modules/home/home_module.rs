use ngyn::macros::module;

use super::home_controller::HomeController;

#[module(controllers = [HomeController])]
pub struct HomeModule {}
