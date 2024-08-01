use std::{any::Any, ptr::NonNull, sync::Arc};

use crate::server::FullResponse;

use super::NgynInjectable;

/// `NgynController` defines the basic structure of a controller in Ngyn.
/// It is designed to be thread-safe.
#[async_trait::async_trait]
pub trait NgynController: NgynInjectable + Any + Sync + Send {
    /// Returns a vector of routes for the controller.
    fn routes(&self) -> Vec<(String, String, String)> {
        vec![]
    }

    fn prefix(&self) -> String {
        '/'.to_string()
    }

    async fn handle(
        &mut self,
        handler: &str,
        cx: &mut crate::server::NgynContext,
        res: &mut crate::server::NgynResponse,
    ) {
        self.inject(cx);
        res.set_status(404);
        res.send(format!("Route not found: {}", handler));
    }
}

impl From<Arc<Box<dyn NgynController>>> for Box<dyn NgynController> {
    fn from(arc: Arc<Box<dyn NgynController>>) -> Self {
        let arc_clone = arc.clone();
        let controller_ref: &dyn NgynController = &**arc_clone;

        let controller_ptr: *const dyn NgynController = controller_ref as *const dyn NgynController;

        let nn_ptr = NonNull::new(controller_ptr as *mut dyn NgynController).unwrap();
        let raw_ptr = nn_ptr.as_ptr();

        unsafe { Box::from_raw(raw_ptr) }
    }
}

/// `NgynControllerHandler` defines placeholders for routing logic of a controller.
pub trait NgynControllerHandler {
    const ROUTES: &'static [(&'static str, &'static str, &'static str)] = &[];

    /// This is for internal use only. It handles the routing logic of the controller.
    #[allow(async_fn_in_trait)]
    async fn __handle_route(
        &mut self,
        _handler: &str,
        _cx: &mut crate::server::NgynContext,
        _res: &mut crate::server::NgynResponse,
    ) {
    }
}
