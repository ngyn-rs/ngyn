#[macro_use]
extern crate ngyn_macros;
use ngyn_macros::AppState;
struct TestState {
    name: String,
}
impl ngyn::shared::server::context::AppState for TestState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
impl<'a> ngyn::shared::server::Transformer<'a> for &'a TestState {
    fn transform(cx: &'a mut ngyn::prelude::NgynContext<'_>) -> Self {
        cx.state::<TestState>().unwrap()
    }
}
impl<'a> ngyn::shared::server::Transformer<'a> for &'a mut TestState {
    fn transform(cx: &'a mut ngyn::prelude::NgynContext<'_>) -> Self {
        cx.state_mut::<TestState>().unwrap()
    }
}
