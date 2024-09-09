#[macro_use]
extern crate ngyn_macros;
use ngyn_macros::injectable;
struct TestService2 {
    name: String,
}
impl ngyn::prelude::NgynInjectable for TestService2 {
    fn new() -> Self {
        TestService2 {
            name: Default::default(),
        }
    }
    fn inject(&mut self, cx: &ngyn::prelude::NgynContext) {}
}
impl Default for TestService2 {
    fn default() -> Self {
        Self::new()
    }
}
struct TestService {
    service2: TestService2,
}
impl ngyn::prelude::NgynInjectable for TestService {
    fn new() -> Self {
        TestService {
            service2: Default::default(),
        }
    }
    fn inject(&mut self, cx: &ngyn::prelude::NgynContext) {
        self.service2.inject(cx);
    }
}
impl Default for TestService {
    fn default() -> Self {
        Self::new()
    }
}
