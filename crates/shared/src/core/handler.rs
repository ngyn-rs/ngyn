use crate::{NgynRequest, NgynResponse};

pub trait Handler: Sync + Send + 'static {
    fn handle(&self, req: &mut NgynRequest, res: &mut NgynResponse);
}

impl<F> Handler for F
where
    F: Fn(&mut NgynRequest, &mut NgynResponse) + Send + Sync + 'static,
{
    fn handle(&self, req: &mut NgynRequest, res: &mut NgynResponse) {
        self(req, res)
    }
}
