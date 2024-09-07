#[macro_use]
extern crate ngyn_macros;

use ngyn_macros::Param;

#[derive(Param)]
struct UserParam {
    id: i32,
    name: String,
}
