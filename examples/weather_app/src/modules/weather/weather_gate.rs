use ngyn::{http::StatusCode, prelude::*, shared::server::Transformer};

#[injectable]
pub struct WeatherGate;

impl NgynGate for WeatherGate {
    async fn can_activate(&self, cx: &mut NgynContext, res: &mut NgynResponse) -> bool {
        let query = Query::transform(cx, res);
        if query.get("location").is_some() {
            return true;
        }
        *res.status_mut() = StatusCode::BAD_REQUEST;
        *res.body_mut() = "Bad Request: location query parameter is required".into();
        // prevent activation of the next components
        false
    }
}
