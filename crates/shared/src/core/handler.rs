use crate::{NgynRequest, NgynResponse};

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
