use std::any::Any;

use rustle_shared::RustleModule;

pub struct RustleFactory {}

impl RustleFactory {
    pub fn create<T: RustleModule<C, P>, C: Any, P: Any>() -> tide::Server<()> {
        let app = tide::new();
        let module = T::new();
        module.get_controllers();
        // .iter().for_each(|controller| {
        //     // app.at(controller.path()).get(controller.handler());
        // });
        // app.with(module);
        app
    }
}