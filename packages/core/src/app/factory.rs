use rustle_shared::RustleModule;

pub struct RustleFactory {}

impl RustleFactory {
    pub fn create<AppModule: RustleModule>() -> tide::Server<()> {
        let mut app = tide::new();
        let module = AppModule::new();
        module.get_controllers().iter().for_each(|controller| {
            for (path, http_method, _handler) in controller.routes() {
                match http_method {
                    // TODO: use an enum for HTTP methods
                    "get" => app.at(path).get(|_| async move { Ok("") }),
                    // TODO: Add other HTTP methods as needed
                    _ => panic!("Unsupported HTTP method"),
                };
            }
        });
        app
    }
}
