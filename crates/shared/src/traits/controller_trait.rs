use hyper::Method;

use crate::server::NgynContext;

use crate::core::Handler;

use super::NgynInjectable;

/// `NgynController` defines the basic structure of a controller in Ngyn.
/// It is designed to be thread-safe.
#[async_trait::async_trait]
pub trait NgynController: NgynInjectable + Sync + Send {
    /// Returns a vector of routes for the controller.
    fn routes(&self) -> Vec<(String, String, String)> {
        vec![]
    }

    fn prefix(&self) -> String {
        '/'.to_string()
    }

    async fn handle(
        &mut self,
        _handler: &str,
        _cx: &mut crate::server::NgynContext,
        _res: &mut crate::server::NgynResponse,
    ) {
    }

    /// Iterates over the routes of the controller.
    ///
    /// # Arguments
    ///
    /// * `f` - The closure to execute for each route.
    fn iter_routes(&self, mut f: impl FnMut(&str, Method, Box<Handler>))
    where
        Self: Clone + 'static,
    {
        for (path, http_method, handler) in self.routes() {
            f(
                path.clone().as_str(),
                hyper::Method::from_bytes(http_method.to_uppercase().as_bytes())
                    .unwrap_or_default(),
                Box::new({
                    let controller = self.clone_box();
                    move |cx: &mut NgynContext, _res| {
                        let controller = controller.clone_box();
                        cx.prepare(controller, handler.clone());
                    }
                }),
            );
        }
    }
}

pub trait CloneBox<T: NgynController + Clone> {
    fn clone_box(&self) -> Box<T>;
}

impl<T: NgynController + Clone> CloneBox<T> for T {
    fn clone_box(&self) -> Box<T> {
        let mut fat_ptr = self as *const T;
        unsafe {
            let data_ptr = &mut fat_ptr as *mut *const T as *mut *mut ();
            assert_eq!(*data_ptr as *const (), self as *const T as *const ());
            *data_ptr = Box::into_raw(Box::new(self.clone())) as *mut ();
        }
        unsafe { Box::from_raw(fat_ptr as *mut T) }
    }
}

impl NgynInjectable for Box<dyn NgynController> {
    fn new() -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn inject(&mut self, cx: &NgynContext) {
        self.as_mut().inject(cx);
    }
}

#[async_trait::async_trait]
impl NgynController for Box<dyn NgynController> {
    fn routes(&self) -> Vec<(String, String, String)> {
        self.as_ref().routes()
    }

    fn prefix(&self) -> String {
        self.as_ref().prefix()
    }

    async fn handle(
        &mut self,
        handler: &str,
        cx: &mut crate::server::NgynContext,
        res: &mut crate::server::NgynResponse,
    ) {
        self.as_mut().handle(handler, cx, res).await;
    }
    fn iter_routes(&self, _f: impl FnMut(&str, Method, Box<Handler>)) {
        unimplemented!()
    }
}

impl Clone for Box<dyn NgynController> {
    fn clone(&self) -> Self {
        self.clone_box()
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
