mod middlewares;
mod modules;

use dotenv::dotenv;
use modules::AppModule;
use ngyn::{server::NgynApplication, NgynFactory, Result};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let app = NgynFactory::<NgynApplication>::create::<AppModule>();

    println!("Starting server at http://127.0.0.1:8080");

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
