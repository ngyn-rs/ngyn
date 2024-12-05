#[macro_use]
extern crate ngyn_macros;
use ngyn_macros::Dto;
struct User {
    id: i32,
    name: String,
}
impl ngyn::shared::server::Transformer<'_> for User {
    fn transform(cx: &mut ngyn::prelude::NgynContext) -> Self {
        ngyn::prelude::Body::transform(cx).json::<User>().unwrap()
    }
}
impl ngyn::shared::server::ToBytes for User {
    fn to_bytes(self) -> ngyn::shared::server::Bytes {
        ngyn::shared::server::Bytes::from(serde_json::to_string(&self).unwrap())
    }
}
