use ngyn::{controller, get, routes, NgynRequest, NgynResponse};

#[controller]
pub struct HomeController;

#[routes]
impl HomeController {
    #[get("/")]
    fn get_home(&self, _req: &mut NgynRequest, res: &mut NgynResponse) {
        res.send("Welcome to the weather app! Try /weather?location=London");
    }
}
