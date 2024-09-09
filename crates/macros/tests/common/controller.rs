#[macro_use]
extern crate ngyn_macros;

use ngyn_macros::{controller, get, inject, injectable, post, routes};

#[injectable]
struct TestService {
    name: String,
}

#[controller]
struct TestController {
    #[inject]
    service: TestService,
}

#[routes]
impl TestController {
    #[get("/")]
    fn index() -> String {
        "Hello, World!".to_string()
    }

    #[post("/")]
    fn create() -> String {
        "Created!".to_string()
    }
}

#[controller("/api")]
struct TestController2 {
    #[inject]
    service: TestService,
}

#[routes]
impl TestController2 {
    #[get("/")]
    fn index() -> String {
        "Hello, World!".to_string()
    }

    #[post("/")]
    fn create() -> String {
        "Created!".to_string()
    }
}

#[controller(inject = "setup")]
struct TestController3 {
    #[inject]
    service: TestService,
}

impl TestController3 {
    fn setup(self, cx: &NgynContext) {
        self.service.inject(cx);
    }
}

#[routes]
impl TestController3 {
    #[get("/")]
    fn index() -> String {
        "Hello, World!".to_string()
    }

    #[post("/")]
    fn create() -> String {
        "Created!".to_string()
    }
}

#[controller(init = "create")]
struct TestController4 {
    #[inject]
    service: TestService,
}

impl TestController4 {
    fn create() -> Self {
        Self {
            service: TestService::new(),
        }
    }
}

#[routes]
impl TestController4 {
    #[get("/")]
    fn index() -> String {
        "Hello, World!".to_string()
    }

    #[post("/")]
    fn create() -> String {
        "Created!".to_string()
    }
}
