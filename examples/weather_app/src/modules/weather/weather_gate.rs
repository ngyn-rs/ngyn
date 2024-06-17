use ngyn::prelude::*;

#[injectable]
pub struct WeatherGate;

impl NgynGate for WeatherGate {
    fn can_activate(&self, cx: &mut NgynContext, res: &mut NgynResponse) -> bool {
        let query = Query::transform(cx, res);
        if query.get("location").is_some() {
            return true;
        }
        res.set_status(400);
        res.send("Bad Request: location query parameter is required");
        false
    }
}
