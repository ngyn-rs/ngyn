use crate::{NgynContext, NgynResponse};

pub type Handler = dyn Fn(&mut NgynContext, &mut NgynResponse) + Send + Sync + 'static;

pub trait RouteHandle: Send + Sync {
    fn into(self) -> Box<Handler>;
}

impl<F> RouteHandle for F
where
    F: Fn(&mut NgynContext, &mut NgynResponse) + Send + Sync + 'static,
{
    fn into(self) -> Box<Handler> {
        Box::new(self)
    }
}
