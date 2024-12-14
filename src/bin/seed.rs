use std::env;

use asg::{
    app_state::config::get_app_key,
    auth::security::hash_password,
    error::AppResult,
    model::{
        repository::ModelRepository,
        users::{User, UserForCreate},
    },
};
use clap::{Parser, Subcommand};
use sqlx::{Pool, Postgres};

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Pool::<Postgres>::connect(&database_url).await?;
    let app_key = get_app_key().expect("APP_KEY not set");

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::All) => {
            seed_users(&pool, &app_key).await?;
            seed_customers(&pool, &app_key).await?;
        }
        Some(Commands::Users) => {
            seed_users(&pool, &app_key).await?;
        }
        Some(Commands::Customers) => {}
        None => println!("No seeding command provided. Use --help for options."),
    }

    Ok(())
}

#[derive(Parser)]
#[command(name = "Database Seeder")]
#[command(about = "Seed the database", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    All,
    Users,
    Customers,
}

async fn seed_users(pool: &Pool<Postgres>, app_key: &[u8]) -> AppResult<()> {
    let users = vec![
        UserForCreate {
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
            hashed_password: hash_password("secret123", app_key)?,
        },
        UserForCreate {
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
            hashed_password: hash_password("secret456", app_key)?,
        },
    ];

    for user in users {
        let email = user.email.clone();
        match User::create(pool, user).await {
            Ok(user) => {
                if let Err(err) = User::set_email_verified_at(pool, user.id).await {
                    eprintln!("Error setting email verified for user {}: {}", user.id, err);
                }
            }
            Err(err) => {
                eprintln!("Error creating user: {}: {}", email, err);
            }
        }
    }

    Ok(())
}

async fn seed_customers(_pool: &Pool<Postgres>, _app_key: &[u8]) -> AppResult<()> {
    Ok(())
}
