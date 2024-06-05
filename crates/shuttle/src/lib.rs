use ngyn_hyper::HyperApplication;
use shuttle_runtime::Error;
use std::net::SocketAddr;

pub struct NgynService(HyperApplication);

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for NgynService {
    /// Takes the app that is returned by the user in their [shuttle_runtime::main] function
    /// and binds to an address passed in by shuttle.
    async fn bind(mut self, addr: SocketAddr) -> Result<(), Error> {
        let _ = self.0.listen(addr).await;
        // .map_err(|err| CustomError::new::<Error>(err.into()))?;
        Ok(())
    }
}

impl From<HyperApplication> for NgynService {
    fn from(app: HyperApplication) -> Self {
        Self(app)
    }
}

pub type ShuttleNgyn = Result<NgynService, Error>;
