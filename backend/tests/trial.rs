mod setup;

#[tokio::test]
async fn testing_test() {
    let app = setup::TestApp::new().await;

    let res = reqwest::get(format!("http://localhost:{}/ingredient", app.addr.port()))
        .await
        .unwrap();

    let body = res.json::<Vec<()>>().await.unwrap();

    assert_eq!(body, vec![]);
}
