#[tokio::main]
async fn main() {
    // Build the application with routes.
    let app = vitr::app();

    // Run the app with hyper, listening globally on port 3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    println!("Running on http://localhost:3000/ (Press CTRL+C to quit)");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
