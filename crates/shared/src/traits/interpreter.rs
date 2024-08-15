use crate::server::NgynResponse;

/// NgynInterpreter is used to interpret a response.
///
/// Sometimes, a response may need to be interpreted before it is sent back to the client.
/// Good examples of this are when a response is expected to be in a certain format, or when
/// the response needs to be modified before it is sent back to the client.
///
/// This trait provides a way to do that.
///
/// ### Examples
///
/// ```rust
/// use ngyn_shared::traits::NgynInterpreter;
/// use ngyn_shared::server::NgynResponse;
///
/// pub struct ResponseInterpreter {}
///
/// #[async_trait::async_trait]
/// impl NgynInterpreter for ResponseInterpreter {
///    async fn interpret(&self, res: &mut NgynResponse) {
///       // Interpret the response here
///   }
/// }
/// ```
#[async_trait::async_trait]
pub trait NgynInterpreter: Send + Sync {
    /// Interprets the response.
    ///
    /// ### Arguments
    ///
    /// * `res` - The response to be interpreted.
    async fn interpret(&self, res: &mut NgynResponse);
}
