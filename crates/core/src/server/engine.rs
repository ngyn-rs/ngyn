use ngyn_shared::{HttpMethod, NgynRequest, NgynResponse};
pub trait Handler: Sync + Send + 'static {
    fn handle(&self, req: NgynRequest, res: NgynResponse) -> NgynResponse;
}

impl<F> Handler for F
where
    F: Fn(NgynRequest, NgynResponse) -> NgynResponse + Send + Sync + 'static,
{
    fn handle(&self, req: NgynRequest, res: NgynResponse) -> NgynResponse {
        self(req, res)
    }
}

pub trait NgynEngine {
    /// Creates a new instance of `NgynService` with a new `Server`
    fn new() -> Self;

    /// Adds a new route to the `NgynService`.
    /// This function is chainable.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that represents the path of the route.
    /// * `method` - An `HttpMethod` that represents the HTTP method of the route.
    /// * `handler` - A closure that takes a `NgynRequest` and a `NgynResponse` and returns a `NgynResponse`.
    ///
    /// # Example
    ///
    /// ```
    /// use ngyn::{server::NgynService, HttpMethod, NgynRequest, NgynResponse};
    ///
    /// let mut server = NgynService::new();
    /// server.route("/", HttpMethod::Get, Box::new(|req: NgynRequest, res: NgynResponse| {
    ///    res.status(200)
    /// }));
    /// ```
    fn route(&mut self, path: &str, method: HttpMethod, handler: Box<impl Handler>) -> &mut Self;
}
