use ngyn::{injectable, NgynGate};

#[injectable]
pub struct WeatherGate;

impl NgynGate for WeatherGate {
    fn check(request: ngyn::NgynRequest) -> bool {
        return true;
    }
}
