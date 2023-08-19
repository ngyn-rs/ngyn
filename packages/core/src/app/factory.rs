use rustle_shared::RustleModule;

pub struct RustleFactory {}

impl RustleFactory {
    pub fn create<AppModule: RustleModule>() -> tide::Server<()> {
        let app = tide::new();
        let module = AppModule::new();
        module.get_controllers().iter().for_each(|_controller| {
            // app.at(controller.path()).get(controller.handler());
        });
        // app.with(module);
        app
    }
}
