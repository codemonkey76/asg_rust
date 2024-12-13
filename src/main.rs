use asg::setup;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Build our application with a route
    let app = setup::initialize_app().await;
    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Server running at http://{}", &addr);
    axum::serve(listener, app).await.unwrap();
}
