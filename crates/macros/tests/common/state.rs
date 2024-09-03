#[macro_use]
extern crate ngyn_macros;

use ngyn_macros::AppState;

#[derive(AppState)]
struct TestState {
    name: String,
}
