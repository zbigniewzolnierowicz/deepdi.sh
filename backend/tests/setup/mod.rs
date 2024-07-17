use std::{net::SocketAddr, time::Duration};

use backend::api::AppBuilder;
use sqlx::{pool::PoolOptions, postgres::PgConnectOptions, PgPool, Postgres};
use testcontainers::{runners::AsyncRunner, ContainerAsync};
use testcontainers_modules::postgres::Postgres as PostgresContainer;
use tokio::net::TcpListener;

pub struct TestApp {
    /// We are storing this, because if this goes out of scope, the container will be cleaned up.
    _db_container: ContainerAsync<PostgresContainer>,
    pub addr: SocketAddr,
    pub db: PgPool,
}

impl TestApp {
    pub async fn new() -> Self {
        let username = "recipes";
        let password = "recipes";
        let database = "recipes";

        let node = PostgresContainer::default()
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

        let db: PgPool = PoolOptions::<Postgres>::new()
            .max_connections(100)
            .idle_timeout(std::time::Duration::from_secs(60))
            .acquire_timeout(Duration::from_secs(60))
            .connect_lazy_with(db_opts);

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
            _db_container: node,
            addr,
            db,
        }
    }

    pub fn get_base(&self, rest: &str) -> String {
        format!("http://{}/{}", self.addr, rest)
    }
}
