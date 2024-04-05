use backend::{api::App, configuration::Settings};
use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let config = Settings::get()?;
    let app = App::new(&config).await?;
    let listener = config.application.get_listener().await?;
    app.serve(listener).await?;

    Ok(())
}
