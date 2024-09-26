use http::StatusCode;
use std::{ptr::NonNull, sync::Arc};

use super::NgynInjectable;

/// `NgynController` defines the basic structure of a controller in Ngyn.
/// Designed for thread safety, it is implemented by controllers that are used to handle requests.
#[async_trait::async_trait]
pub trait NgynController: NgynInjectable + Sync + Send {
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
/// And we do this because controllers are shared across threads and an arc guarantees
/// that the controller is not dropped until all references to it are dropped.
///
/// When working with controllers, you'd quickly notice that Ngyn allows you to define routes that require mutable access to the controller.
/// For instance, take this sample controller:
/// ```rust ignore
/// #[controller]
/// struct TestController;
///
/// #[routes]
/// impl TestController {
///    #[get("/")]
///    async fn index(&mut self) -> String {
///      "Hello, World!".to_string()
///    }
/// }
/// ```
///
/// In the above example, the `index` method requires mutable access to the controller. This pattern, though not encouraged (check app states), is allowed in Ngyn.
/// You could for instance create a localized state in the controller that is only accessible to the controller and its routes.
/// The way Ngyn allows this without performance overhead is through a specialized `Arc -> Box` conversion that only works so well becasue of how Ngyn is designed.
///
/// HOW DOES IT WORK?
///
/// ```text
/// +-----------------+        +-----------------+        +-----------------+
/// | Arc<Box<Ctrl>>  |        | Arc<Box<Ctrl>>  |        | Arc<Box<Ctrl>>  |
/// +-----------------+        +-----------------+        +-----------------+
///        |                          |                          |
/// +-----------------+        +-----------------+        +-----------------+
/// | &Box<Ctrl>      |        | &Box<Ctrl>      |        | &Box<Ctrl>      |
/// +-----------------+        +-----------------+        +-----------------+
///        |                          |                          |
/// +-----------------+        +-----------------+        +-----------------+
/// | &mut Ctrl       |        | &mut Ctrl       |        | &mut Ctrl       |
/// +-----------------+        +-----------------+        +-----------------+
///        |                          |                          |
/// +-----------------+        +-----------------+        +-----------------+
/// | *mut Ctrl       |        | *mut Ctrl       |        | *mut Ctrl       |
/// +-----------------+        +-----------------+        +-----------------+
///        |                          |                          |
/// +-----------------+        +-----------------+        +-----------------+
/// | Box<Ctrl>       |        | Box<Ctrl>       |        | Box<Ctrl>       |
/// +-----------------+        +-----------------+        +-----------------+
/// 
/// ```
///
///
/// When a controller is created, we box it and then wrap it in an Arc. This way, the controller is converted to a trait object and can be shared across threads.
/// The trait object is what allows us to call the controller's methods from the server. But when we need mutable access to the controller, we convert it back to a Box.
/// Rather than making use of a mutex, what we do is get the raw pointer of the initial controller, ensure it's not null, and then convert it back to a Box.
///
/// # Panics
/// Panics if the controller has been dropped. This should never happen unless the controller is dropped manually.
impl From<Arc<Box<dyn NgynController>>> for Box<dyn NgynController> {
    fn from(controller_arc: Arc<Box<dyn NgynController>>) -> Self {
        let controller_ref: &dyn NgynController = &**controller_arc;
        let controller_ptr: *const dyn NgynController = controller_ref as *const dyn NgynController;

        // SAFETY: controller_ptr is not null, it is safe to convert it to a NonNull pointer, this way we can safely convert it back to a Box
        let nn_ptr = NonNull::new(controller_ptr as *mut dyn NgynController)
            .expect("Controller has been dropped, ensure it is being cloned correctly.");
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
