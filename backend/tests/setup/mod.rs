use std::net::SocketAddr;

use backend::api::AppBuilder;
use sqlx::{postgres::PgConnectOptions, PgPool};
use testcontainers::{runners::AsyncRunner, ContainerAsync};
use testcontainers_modules::postgres::Postgres;
use tokio::net::TcpListener;

pub struct TestApp {
    /// We are storing this, because if this goes out of scope, the container will be cleaned up.
    _db_image: ContainerAsync<Postgres>,
    pub addr: SocketAddr,
}

impl TestApp {
    pub async fn new() -> Self {
        let node = Postgres::default().start().await;

        dbg!(node.id());

        let db_opts = PgConnectOptions::new()
            .host(&node.get_host().await.to_string())
            .port(node.get_host_port_ipv4(5432).await)
            .username("postgres")
            .password("postgres")
            .database("postgres");

        let db: PgPool = PgPool::connect_with(db_opts).await.unwrap();

        sqlx::migrate!().run(&db).await.unwrap();

        let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let app = AppBuilder::new()
            .with_postgres_database(db)
            .build()
            .unwrap();

        tokio::spawn(async move {
            app.serve(listener).await.unwrap();
        });

        TestApp {
            _db_image: node,
            addr,
        }
    }
}
