#[macro_use]
extern crate ngyn_macros;

use ngyn_macros::injectable;

#[injectable]
struct TestService2 {
    name: String,
}

#[injectable]
struct TestService {
    #[inject]
    service2: TestService2,
}
