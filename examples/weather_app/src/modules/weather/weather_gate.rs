use ngyn::prelude::*;

#[injectable]
pub struct WeatherGate;

impl NgynGate for WeatherGate {
    fn can_activate(self, _request: &mut NgynRequest) -> bool {
        true
    }
}
