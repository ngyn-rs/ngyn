#[async_trait::async_trait]
/// `NgynController` defines the basic structure of a controller in Ngyn.
/// It is designed to be thread-safe.
pub trait NgynController: Send + Sync {
    /// Creates a new instance of the controller.
    /// This is for internal use only.
    fn new(middlewares: Vec<std::sync::Arc<dyn super::NgynMiddleware>>) -> Self
    where
        Self: Sized;

    /// Returns a vector of routes for the controller.
    fn routes(&self) -> Vec<(String, String, String)>;

    /// This is for internal use only. It handles the routing logic of the controller.
    async fn handle(
        &self,
        handler: &str,
        req: &mut crate::NgynRequest,
        res: &mut crate::NgynResponse,
    );
}

#[async_trait::async_trait]
/// `NgynControllerRoutePlaceholder` defines placeholders for routing logic of a controller.
pub trait NgynControllerRoutePlaceholder {
    #[allow(non_upper_case_globals)]
    const routes: &'static [(&'static str, &'static str, &'static str)];
    async fn __handle_route(
    	&self,
        handler: &str,
        req: &mut crate::NgynRequest,
        res: &mut crate::NgynResponse,
    );
}
