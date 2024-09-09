#[macro_use]
extern crate ngyn_macros;
use ngyn_macros::module;
struct AppModule {}
impl ngyn::shared::traits::NgynModule for AppModule {
    fn new() -> Self {
        AppModule {}
    }
    fn get_controllers(
        &self,
    ) -> Vec<std::sync::Arc<Box<dyn ngyn::shared::traits::NgynController + 'static>>> {
        use ngyn::shared::traits::NgynInjectable;
        let mut controllers: Vec<
            std::sync::Arc<Box<dyn ngyn::shared::traits::NgynController + 'static>>,
        > = ::alloc::vec::Vec::new();
        controllers
    }
}
struct TestModule {}
impl ngyn::shared::traits::NgynModule for TestModule {
    fn new() -> Self {
        TestModule {}
    }
    fn get_controllers(
        &self,
    ) -> Vec<std::sync::Arc<Box<dyn ngyn::shared::traits::NgynController + 'static>>> {
        use ngyn::shared::traits::NgynInjectable;
        let mut controllers: Vec<
            std::sync::Arc<Box<dyn ngyn::shared::traits::NgynController + 'static>>,
        > = <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                std::sync::Arc::new(
                    Box::new(TestController::new())
                        as Box<dyn ngyn::shared::traits::NgynController + 'static>,
                ),
            ]),
        );
        controllers
    }
}
