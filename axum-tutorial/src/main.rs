use axum::{Router, routing::get};

mod routes;

#[tokio::main]
async fn main() {
    const PORT: u16 = 3000;
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/movies", routes::movies::routes()); // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", PORT))
        .await
        .unwrap();
    println!("Server is running on port: {}", PORT);
    axum::serve(listener, app).await.unwrap();
}
