use super::NgynInjectable;

#[async_trait::async_trait]
/// `NgynController` defines the basic structure of a controller in Ngyn.
/// It is designed to be thread-safe.
pub trait NgynController: NgynInjectable + Sync {
    /// Returns a vector of routes for the controller.
    fn routes(&self) -> Vec<(String, String, String)>;

    /// This is for internal use only. It handles the routing logic of the controller.
    async fn handle(
        &self,
        handler: &str,
        cx: &mut crate::server::NgynContext,
        res: &mut crate::server::NgynResponse,
    );
}

#[async_trait::async_trait]
/// `NgynControllerHandler` defines placeholders for routing logic of a controller.
pub trait NgynControllerHandler {
    const ROUTES: &'static [(&'static str, &'static str, &'static str)] = &[];

    async fn __handle_route(
        &self,
        _handler: &str,
        _cx: &mut crate::server::NgynContext,
        _res: &mut crate::server::NgynResponse,
    ) {
    }
}
