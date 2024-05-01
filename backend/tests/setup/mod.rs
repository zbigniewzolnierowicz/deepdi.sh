use std::net::SocketAddr;

use backend::api::AppBuilder;
use sqlx::{postgres::PgConnectOptions, PgPool};
use testcontainers::{runners::AsyncRunner, ContainerAsync};
use testcontainers_modules::postgres::Postgres;
use tokio::net::TcpListener;

pub struct TestApp {
    /// We are storing this, because if this goes out of scope, the container will be cleaned up.
    _db: ContainerAsync<Postgres>,
    pub addr: SocketAddr,
    pub db: PgPool,
}

impl TestApp {
    pub async fn new() -> Self {
        let username = "recipes";
        let password = "recipes";
        let database = "recipes";

        let node = Postgres::default()
            .with_user(username)
            .with_password(password)
            .with_db_name(database)
            .start()
            .await;

        let db_opts = PgConnectOptions::new()
            .host(&node.get_host().await.to_string())
            .port(node.get_host_port_ipv4(5432).await)
            .username(username)
            .password(password)
            .database(database);

        let db: PgPool = PgPool::connect_with(db_opts).await.unwrap();

        sqlx::migrate!().run(&db).await.unwrap();

        let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let app = AppBuilder::new()
            .with_postgres_database(db.clone())
            .build()
            .unwrap();

        tokio::spawn(async move {
            app.serve(listener).await.unwrap();
        });

        TestApp {
            _db: node,
            addr,
            db,
        }
    }

    pub fn get_base(&self, rest: &str) -> String {
        format!("http://{}/{}", self.addr, rest)
    }
}
