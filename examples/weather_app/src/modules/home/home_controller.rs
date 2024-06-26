use ngyn::prelude::*;

#[controller]
pub struct HomeController;

#[routes]
impl HomeController {
    #[get("/")]
    fn get_home(&self) -> &str {
        "Welcome to the weather app! Try /weather?location=London"
    }
}
