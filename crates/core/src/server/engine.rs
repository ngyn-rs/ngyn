use ngyn_shared::{HttpMethod, NgynRequest, NgynResponse};

pub trait Handler: Sync + Send + 'static {
    fn handle(&self, req: &NgynRequest, res: &mut NgynResponse);
}

impl<F> Handler for F
where
    F: Fn(&NgynRequest, &mut NgynResponse) + Send + Sync + 'static,
{
    fn handle(&self, req: &NgynRequest, res: &mut NgynResponse) {
        self(req, res)
    }
}

pub trait NgynEngine {
    /// Creates a new instance of `NgynApplication` with a new `Server`
    fn new() -> Self;

    /// Adds a new route to the `NgynApplication`.
    /// This function is chainable.
    ///
    /// ### Arguments
    ///
    /// * `path` - A string slice that represents the path of the route.
    /// * `method` - An `HttpMethod` that represents the HTTP method of the route.
    /// * `handler` - A closure that takes a `NgynRequest` and a `NgynResponse` and returns a `NgynResponse`.
    ///
    /// ### Example
    ///
    /// ```
    /// use ngyn::{server::{NgynApplication, NgynEngine}, HttpMethod, NgynRequest, NgynResponse};
    ///
    /// let mut server = NgynApplication::new();
    /// server.route("/", HttpMethod::Get, Box::new(|req: &NgynRequest, res: &mut NgynResponse| {
    ///    res.status(200);
    /// }));
    /// ```
    fn route(&mut self, path: &str, method: HttpMethod, handler: Box<impl Handler>) -> &mut Self;
}
