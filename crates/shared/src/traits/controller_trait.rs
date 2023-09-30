/// `NgynControllerInit` is a trait that defines the basic structure of a controller initializer in Ngyn.
pub trait NgynControllerInit: Send + Sync {
    /// Creates a new instance of the controller.
    /// This is for internal use only.
    fn new(middlewares: Vec<std::sync::Arc<dyn super::NgynMiddleware>>) -> Self;
}

#[tide::utils::async_trait]
/// `NgynController` is a trait that defines the basic structure of a controller in Ngyn.
/// It is designed to be thread-safe.
pub trait NgynController: Send + Sync {
    /// Returns the name of the controller.
    fn name(&self) -> &str;

    /// Returns a vector of routes for the controller.
    fn get_routes(&self) -> Vec<(String, String, String)>;

    /// Returns a `NgynResponse` for the controller.
    /// This is for internal use only.
    async fn handle(
        &self,
        handler: String,
        req: crate::NgynRequest,
        res: crate::NgynResponse,
    ) -> crate::NgynResponse;
}
