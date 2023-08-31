use std::vec;

use nject::provider;
use rustle_shared::{RustleRequest, RustleResponse};

/// `RustleProvider` is a struct that acts as a provider in Rustle.
/// This single struct is used to inject all the dependencies.
/// It is for internal use only.
#[provider]
pub struct RustleProvider;

impl
    nject::Provider<
        '_,
        Vec<(
            String,
            String,
            Box<dyn Fn(RustleRequest, RustleResponse) -> RustleResponse + Send + Sync>,
        )>,
    > for RustleProvider
{
    fn provide(
        &self,
    ) -> Vec<(
        String,
        String,
        Box<dyn Fn(RustleRequest, RustleResponse) -> RustleResponse + Send + Sync>,
    )> {
        vec![]
    }
}
