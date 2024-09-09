#[macro_use]
extern crate ngyn_macros;
use ngyn_macros::{controller, get, inject, injectable, post, routes};
struct TestService {
    name: String,
}
impl ngyn::prelude::NgynInjectable for TestService {
    fn new() -> Self {
        TestService {
            name: Default::default(),
        }
    }
    fn inject(&mut self, cx: &ngyn::prelude::NgynContext) {}
}
impl Default for TestService {
    fn default() -> Self {
        Self::new()
    }
}
struct TestController {
    service: TestService,
}
impl ngyn::shared::traits::NgynInjectable for TestController {
    fn new() -> Self {
        TestController {
            service: Default::default(),
        }
    }
    fn inject(&mut self, cx: &ngyn::prelude::NgynContext) {
        self.service.inject(cx);
    }
}
#[ngyn::prelude::async_trait]
impl ngyn::shared::traits::NgynController for TestController {
    fn routes(&self) -> Vec<(String, String, String)> {
        use ngyn::shared::traits::NgynControllerHandler;
        Self::ROUTES
            .iter()
            .map(|(path, method, handler)| {
                let path = {
                    let res = ::alloc::fmt::format(format_args!("{0}", path));
                    res
                }
                    .trim_start_matches('/')
                    .to_string();
                let prefix = "".to_string().trim_end_matches('/').to_string();
                (
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("{0}/{1}", prefix, path),
                        );
                        res
                    },
                    method.to_string(),
                    handler.to_string(),
                )
            })
            .collect()
    }
    fn prefix(&self) -> String {
        "".to_string()
    }
    async fn handle(
        &mut self,
        handler: &str,
        cx: &mut ngyn::prelude::NgynContext,
        res: &mut ngyn::prelude::NgynResponse,
    ) {
        use ngyn::shared::traits::NgynControllerHandler;
        self.inject(cx);
        self.__handle_route(handler, cx, res).await;
    }
}
impl TestController {
    const ROUTES: &'static [(&'static str, &'static str, &'static str)] = &[
        ("/", "GET", "index"),
        ("/", "POST", "create"),
    ];
    fn index() -> String {
        "Hello, World!".to_string()
    }
    fn create() -> String {
        "Created!".to_string()
    }
}
impl TestController {
    async fn __handle_route(
        &mut self,
        handler: &str,
        cx: &mut ngyn::prelude::NgynContext,
        res: &mut ngyn::prelude::NgynResponse,
    ) {
        use ngyn::shared::server::ToBytes;
        *res.status_mut() = ngyn::http::StatusCode::CREATED;
        match handler {
            "index" => {
                let body = self.index();
                *res.body_mut() = body.to_bytes().into();
            }
            "create" => {
                let body = self.create();
                *res.body_mut() = body.to_bytes().into();
            }
            _ => {}
        }
    }
}
struct TestController2 {
    service: TestService,
}
impl ngyn::shared::traits::NgynInjectable for TestController2 {
    fn new() -> Self {
        TestController2 {
            service: Default::default(),
        }
    }
    fn inject(&mut self, cx: &ngyn::prelude::NgynContext) {
        self.service.inject(cx);
    }
}
#[ngyn::prelude::async_trait]
impl ngyn::shared::traits::NgynController for TestController2 {
    fn routes(&self) -> Vec<(String, String, String)> {
        use ngyn::shared::traits::NgynControllerHandler;
        Self::ROUTES
            .iter()
            .map(|(path, method, handler)| {
                let path = {
                    let res = ::alloc::fmt::format(format_args!("{0}", path));
                    res
                }
                    .trim_start_matches('/')
                    .to_string();
                let prefix = {
                    let res = ::alloc::fmt::format(format_args!("{0}", "/api"));
                    res
                }
                    .trim_end_matches('/')
                    .to_string();
                (
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("{0}/{1}", prefix, path),
                        );
                        res
                    },
                    method.to_string(),
                    handler.to_string(),
                )
            })
            .collect()
    }
    fn prefix(&self) -> String {
        {
            let res = ::alloc::fmt::format(format_args!("{0}", "/api"));
            res
        }
    }
    async fn handle(
        &mut self,
        handler: &str,
        cx: &mut ngyn::prelude::NgynContext,
        res: &mut ngyn::prelude::NgynResponse,
    ) {
        use ngyn::shared::traits::NgynControllerHandler;
        self.inject(cx);
        self.__handle_route(handler, cx, res).await;
    }
}
impl TestController2 {
    const ROUTES: &'static [(&'static str, &'static str, &'static str)] = &[
        ("/", "GET", "index"),
        ("/", "POST", "create"),
    ];
    fn index() -> String {
        "Hello, World!".to_string()
    }
    fn create() -> String {
        "Created!".to_string()
    }
}
impl TestController2 {
    async fn __handle_route(
        &mut self,
        handler: &str,
        cx: &mut ngyn::prelude::NgynContext,
        res: &mut ngyn::prelude::NgynResponse,
    ) {
        use ngyn::shared::server::ToBytes;
        *res.status_mut() = ngyn::http::StatusCode::CREATED;
        match handler {
            "index" => {
                let body = self.index();
                *res.body_mut() = body.to_bytes().into();
            }
            "create" => {
                let body = self.create();
                *res.body_mut() = body.to_bytes().into();
            }
            _ => {}
        }
    }
}
struct TestController3 {
    service: TestService,
}
impl ngyn::shared::traits::NgynInjectable for TestController3 {
    fn new() -> Self {
        TestController3 {
            service: Default::default(),
        }
    }
    fn inject(&mut self, cx: &ngyn::prelude::NgynContext) {
        self.service.inject(cx);
        TestController3::setup()
    }
}
#[ngyn::prelude::async_trait]
impl ngyn::shared::traits::NgynController for TestController3 {
    fn routes(&self) -> Vec<(String, String, String)> {
        use ngyn::shared::traits::NgynControllerHandler;
        Self::ROUTES
            .iter()
            .map(|(path, method, handler)| {
                let path = {
                    let res = ::alloc::fmt::format(format_args!("{0}", path));
                    res
                }
                    .trim_start_matches('/')
                    .to_string();
                let prefix = "".to_string().trim_end_matches('/').to_string();
                (
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("{0}/{1}", prefix, path),
                        );
                        res
                    },
                    method.to_string(),
                    handler.to_string(),
                )
            })
            .collect()
    }
    fn prefix(&self) -> String {
        "".to_string()
    }
    async fn handle(
        &mut self,
        handler: &str,
        cx: &mut ngyn::prelude::NgynContext,
        res: &mut ngyn::prelude::NgynResponse,
    ) {
        use ngyn::shared::traits::NgynControllerHandler;
        self.inject(cx);
        self.__handle_route(handler, cx, res).await;
    }
}
impl TestController3 {
    fn setup(self, cx: &NgynContext) {
        self.service.inject(cx);
    }
}
impl TestController3 {
    const ROUTES: &'static [(&'static str, &'static str, &'static str)] = &[
        ("/", "GET", "index"),
        ("/", "POST", "create"),
    ];
    fn index() -> String {
        "Hello, World!".to_string()
    }
    fn create() -> String {
        "Created!".to_string()
    }
}
impl TestController3 {
    async fn __handle_route(
        &mut self,
        handler: &str,
        cx: &mut ngyn::prelude::NgynContext,
        res: &mut ngyn::prelude::NgynResponse,
    ) {
        use ngyn::shared::server::ToBytes;
        *res.status_mut() = ngyn::http::StatusCode::CREATED;
        match handler {
            "index" => {
                let body = self.index();
                *res.body_mut() = body.to_bytes().into();
            }
            "create" => {
                let body = self.create();
                *res.body_mut() = body.to_bytes().into();
            }
            _ => {}
        }
    }
}
struct TestController4 {
    service: TestService,
}
impl ngyn::shared::traits::NgynInjectable for TestController4 {
    fn new() -> Self {
        TestController4::create()
    }
    fn inject(&mut self, cx: &ngyn::prelude::NgynContext) {
        self.service.inject(cx);
    }
}
#[ngyn::prelude::async_trait]
impl ngyn::shared::traits::NgynController for TestController4 {
    fn routes(&self) -> Vec<(String, String, String)> {
        use ngyn::shared::traits::NgynControllerHandler;
        Self::ROUTES
            .iter()
            .map(|(path, method, handler)| {
                let path = {
                    let res = ::alloc::fmt::format(format_args!("{0}", path));
                    res
                }
                    .trim_start_matches('/')
                    .to_string();
                let prefix = "".to_string().trim_end_matches('/').to_string();
                (
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("{0}/{1}", prefix, path),
                        );
                        res
                    },
                    method.to_string(),
                    handler.to_string(),
                )
            })
            .collect()
    }
    fn prefix(&self) -> String {
        "".to_string()
    }
    async fn handle(
        &mut self,
        handler: &str,
        cx: &mut ngyn::prelude::NgynContext,
        res: &mut ngyn::prelude::NgynResponse,
    ) {
        use ngyn::shared::traits::NgynControllerHandler;
        self.inject(cx);
        self.__handle_route(handler, cx, res).await;
    }
}
impl TestController4 {
    fn create() -> Self {
        Self {
            service: TestService::new(),
        }
    }
}
impl TestController4 {
    const ROUTES: &'static [(&'static str, &'static str, &'static str)] = &[
        ("/", "GET", "index"),
        ("/", "POST", "create"),
    ];
    fn index() -> String {
        "Hello, World!".to_string()
    }
    fn create() -> String {
        "Created!".to_string()
    }
}
impl TestController4 {
    async fn __handle_route(
        &mut self,
        handler: &str,
        cx: &mut ngyn::prelude::NgynContext,
        res: &mut ngyn::prelude::NgynResponse,
    ) {
        use ngyn::shared::server::ToBytes;
        *res.status_mut() = ngyn::http::StatusCode::CREATED;
        match handler {
            "index" => {
                let body = self.index();
                *res.body_mut() = body.to_bytes().into();
            }
            "create" => {
                let body = self.create();
                *res.body_mut() = body.to_bytes().into();
            }
            _ => {}
        }
    }
}
