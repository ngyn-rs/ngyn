use crate::{NgynContext, NgynResponse};

pub type Handler = dyn Fn(&mut NgynContext, &mut NgynResponse) + Send + Sync + 'static;
