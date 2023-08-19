use nject::provider;

/// `RustleProvider` is a struct that acts as a provider in Rustle.
/// This single struct is used to inject all the dependencies.
/// It is for internal use only.
#[provider]
pub struct RustleProvider;
