#[macro_use]
extern crate ngyn_macros;

use ngyn_macros::module;

#[module]
struct AppModule;

#[module(controllers = [TestController])]
struct TestModule;
