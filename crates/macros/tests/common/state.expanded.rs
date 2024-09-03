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
