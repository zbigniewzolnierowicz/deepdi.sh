use backend::{api::App, configuration::Settings};
use color_eyre::Result;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let config = Settings::get()?;
    let db = PgPool::connect_lazy_with(config.database.with_db());
    let app = App::with_db(db).await?;
    let listener = config.application.get_listener().await?;
    app.serve(listener).await?;

    Ok(())
}
