#[macro_use]
extern crate ngyn_macros;

use ngyn_macros::Dto;

#[derive(Dto)]
struct User {
    id: i32,
    name: String,
}
