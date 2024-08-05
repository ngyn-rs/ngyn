use crate::server::NgynResponse;

/// NgynInterpreter is used to interpret a response.
///
/// Sometimes, a response may need to be interpreted before it is sent back to the client.
/// This trait provides a way to do that.
pub trait NgynInterpreter: Send + Sync {
    fn interpret(&self, res: &mut NgynResponse) -> NgynResponse;
}
