use nject::provider;

/// `NgynProvider` is a struct that acts as a provider in Ngyn.
/// This single struct is used to inject all the dependencies.
/// It is for internal use only.
#[provider]
pub struct NgynProvider;

impl nject::Provider<'_, Vec<(String, String, String)>> for NgynProvider {
    fn provide(&self) -> Vec<(String, String, String)> {
        vec![]
    }
}
