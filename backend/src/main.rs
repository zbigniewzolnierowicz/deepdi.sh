use color_eyre::Result;
use backend::{api::App, configuration::Settings};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let config = Settings::get()?;
    let app = App::new(config.database.with_db()).await?;
    let listener = config.application.get_listener().await?;
    app.serve(listener).await?;

    Ok(())
}
