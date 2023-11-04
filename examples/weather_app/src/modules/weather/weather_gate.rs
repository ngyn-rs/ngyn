use ngyn::{injectable, NgynGate};

#[injectable]
pub struct WeatherGate;

impl NgynGate for WeatherGate {
    fn can_activate(self, _request: &ngyn::NgynRequest) -> bool {
        true
    }
}
