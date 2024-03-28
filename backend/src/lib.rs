use axum::{routing::get, Router};

pub struct App {
    router: Router,
}

impl App {
    pub fn new() -> Self {
        let router = Router::new().route("/", get(|| async { "Hello, world!" }));

        App { router }
    }

    pub async fn serve(self, listener: tokio::net::TcpListener) -> color_eyre::Result<()> {
        axum::serve(listener, self.router).await?;
        Ok(())
    }
}
