use http::StatusCode;
use std::{any::Any, ptr::NonNull, sync::Arc};

use super::NgynInjectable;

/// `NgynController` defines the basic structure of a controller in Ngyn.
/// Designed for thread safety, it is implemented by controllers that are used to handle requests.
#[async_trait::async_trait]
pub trait NgynController: NgynInjectable + Any + Sync + Send {
    /// Returns a vector of routes for the controller.
    fn routes(&self) -> Vec<(String, String, String)> {
        vec![]
    }

    /// Returns the prefix for the controller.
    fn prefix(&self) -> String {
        '/'.to_string()
    }

    /// used internally to handle the routing logic of the controller.
    async fn handle(
        &mut self,
        handler: &str,
        cx: &mut crate::server::NgynContext,
        res: &mut crate::server::NgynResponse,
    ) {
        self.inject(cx);
        *res.status_mut() = StatusCode::NOT_FOUND;
        *res.body_mut() = format!("Route not found: {}", handler).into();
    }
}

/// In Ngyn, controllers are stored as `Arc<Box<dyn NgynController>>`.
/// This is because controllers are shared across threads and need to be cloned easily.
///
/// Here's how we convert an `Arc<Box<dyn NgynController>>` to a `Box<dyn NgynController>`.
/// This conversion allows us to mutably borrow the controller and handle routing logic.
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

/// `NgynControllerHandler` is an internal trait that defines placeholders for routing logic of a controller.
pub trait NgynControllerHandler: NgynController {
    const ROUTES: &'static [(&'static str, &'static str, &'static str)] = &[];

    /// This is for internal use only. It handles the routing logic of the controller.
    #[allow(async_fn_in_trait)]
    async fn __handle_route(
        &mut self,
        _handler: &str,
        _cx: &mut crate::server::NgynContext,
        _res: &mut crate::server::NgynResponse,
    ) {
        // do nothing
    }
}

/// implement for all types that implement `NgynController`
impl<T: NgynController> NgynControllerHandler for T {}
