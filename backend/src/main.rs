use color_eyre::Result;
use backend::App;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 8111)).await?;
    let app = App::new();
    app.serve(listener).await?;

    Ok(())
}
