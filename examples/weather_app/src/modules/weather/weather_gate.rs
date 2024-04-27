use ngyn::prelude::*;

#[injectable]
pub struct WeatherGate;

impl NgynGate for WeatherGate {
    fn can_activate(self, _cx: &mut NgynContext, _res: &mut NgynResponse) -> bool {
        true
    }
}
