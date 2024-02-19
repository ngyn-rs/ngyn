use ngyn::prelude::*;

#[controller]
pub struct HomeController;

#[routes]
impl HomeController {
    #[get("/")]
    fn get_home(&self, query: Query) -> String {
    	println!("{:?}", query.get("location"));
        "Welcome to the weather app! Try /weather?location=London".to_string()
    }
}
