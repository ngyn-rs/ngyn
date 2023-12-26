use ngyn::{injectable, NgynGate};

#[injectable]
pub struct WeatherGate;

impl NgynGate for WeatherGate {
    fn can_activate(self, _request: &mut ngyn::NgynRequest) -> bool {
        true
    }
}
