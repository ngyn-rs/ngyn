use crate::{NgynContext, NgynResponse};

pub trait Handler: Sync + Send + 'static {
    fn handle(&self, cx: &mut NgynContext, res: &mut NgynResponse);
}

impl<F> Handler for F
where
    F: Fn(&mut NgynContext, &mut NgynResponse) + Send + Sync + 'static,
{
    fn handle(&self, cx: &mut NgynContext, res: &mut NgynResponse) {
        self(cx, res)
    }
}
