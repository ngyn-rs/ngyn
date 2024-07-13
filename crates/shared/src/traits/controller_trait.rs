use super::NgynInjectable;

/// `NgynController` defines the basic structure of a controller in Ngyn.
/// It is designed to be thread-safe.
#[async_trait::async_trait]
pub trait NgynController: NgynInjectable + Sync + Send {
    /// Returns a vector of routes for the controller.
    fn routes(&self) -> Vec<(String, String, String)>;

    fn prefix(&self) -> String {
        "/".to_string()
    }

    async fn handle(
        &mut self,
        _handler: &str,
        _cx: &mut crate::server::NgynContext,
        _res: &mut crate::server::NgynResponse,
    ) {
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
