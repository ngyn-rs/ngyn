use ngyn::{injectable, NgynGate};

#[injectable]
pub struct WeatherGate;

impl NgynGate for WeatherGate {
    fn check(self, _request: &ngyn::NgynRequest) -> bool {
        true
    }
}
