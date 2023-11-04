use ngyn::{controller, get, NgynRequest, NgynResponse};

#[controller("get_home")]
pub struct HomeController {}

impl HomeController {
    #[get("/")]
    fn get_home(self, _req: &NgynRequest, res: &mut NgynResponse) -> NgynResponse {
        res.body("Welcome to the weather app! Try /weather?location=London");
        res.clone()
    }
}
