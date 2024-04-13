use crate::{NgynContext, NgynResponse};

pub type Handler = dyn FnOnce(&mut NgynContext, &mut NgynResponse) + Send + Sync + 'static;
