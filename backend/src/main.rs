use axum::routing::get;
use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, world!" }));
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 8111)).await?;

    axum::serve(listener, app).await?;
    Ok(())
}
