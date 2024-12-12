use asg::{app_state, setup};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = setup::initialize_app().await;

    let app_key = app_state::config::get_app_key();
    println!("Hello, world!");
}
