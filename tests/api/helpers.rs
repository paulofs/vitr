use tokio::spawn;

pub struct TestApp {
    pub address: String,
}

pub async fn spawn_app() -> TestApp {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:0").await.unwrap();

    let application_port = listener.local_addr().unwrap().port();
    let application_address = format!("http://127.0.0.1:{application_port}");

    spawn(axum::serve(listener, vitr::app()).into_future());

    TestApp {
        address: application_address,
    }
}
