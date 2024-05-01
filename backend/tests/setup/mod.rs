use std::net::SocketAddr;

use backend::api::AppBuilder;
use sqlx::PgPool;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;
use tokio::net::TcpListener;

async fn db_setup() -> PgPool {
    let node = Postgres::default().start().await;

    // prepare connection string
    let connection_string = &format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        node.get_host_port_ipv4(5432).await
    );

    let db: PgPool = PgPool::connect(connection_string).await.unwrap();

    sqlx::migrate!().run(&db).await.unwrap();

    db
}

pub async fn setup() -> SocketAddr {
    let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let pool = db_setup().await;

    // A check for checking if the database *actually* connected
    let res = sqlx::query!("SELECT 1 as test").fetch_one(&pool).await.unwrap();
    dbg!(res);

    tokio::spawn(async move {
        let app = AppBuilder::new()
            .with_postgres_database(pool)
            .build()
            .unwrap();

        app.serve(listener).await.unwrap();
    });

    addr
}
