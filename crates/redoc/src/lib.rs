use ngyn::shared::{core::NgynEngine, traits::NgynController};
use std::sync::{Arc, Mutex};

pub mod routing;

pub trait NgynEngineReDoc: NgynEngine {
    fn use_redoc(&mut self, config: routing::ReDocConfig) {
        let controller = routing::ReDocModule::with_config(config);
        let controller = Arc::new(Mutex::new(vec![
            Box::new(controller) as Box<dyn NgynController>
        ]));
        self.load_controller(controller);
    }
}

impl<T: NgynEngine> NgynEngineReDoc for T {}
