use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let client = reqwest::Client::new()
        .get(format!("{}/health_check", app.address))
        .send()
        .await
        .unwrap();

    // Assert
    assert!(client.status().is_success());
    assert_eq!(Some(0), client.content_length());
}
