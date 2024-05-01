mod setup;

#[tokio::test]
async fn testing_test() {
    let addr = setup::setup().await;

    let client = reqwest::Client::builder().build().unwrap();

    let res = client
        .get(format!("http://localhost:{}/ingredient", addr.port()))
        .send()
        .await
        .unwrap();

    assert_eq!(res.text().await.unwrap(), "[]");
}
