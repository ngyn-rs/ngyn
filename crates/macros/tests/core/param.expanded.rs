#[macro_use]
extern crate ngyn_macros;
use ngyn_macros::Param;
struct UserParam {
    id: i32,
    name: String,
}
impl ngyn::shared::server::Transformer<'_> for UserParam {
    fn transform(cx: &mut ngyn::prelude::NgynContext<'_>) -> Self {
        let param = ngyn::shared::server::Param::transform(cx);
        UserParam {
            id: param.get("id").unwrap_or_default(),
            name: param.get("name").unwrap_or_default(),
        }
    }
}
