use std::env;

use asg::{error::AppResult, model::users::User};
use clap::{Parser, Subcommand};
use sqlx::{Pool, Postgres};

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Pool::<Postgres>::connect(&database_url).await?;

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::All) => {
            seed_users(&pool).await?;
            seed_customers(&pool).await?;
        }
        Some(Commands::Users) => {
            seed_users(&pool).await?;
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

async fn seed_users(pool: &Pool<Postgres>) -> AppResult<()> {
    let users = vec![
        ("Alice", "alice@example.com", "secret123"),
        ("Bob", "bob@example.com", "secret456"),
    ];

    for (name, email, password) in users {
        match User::create(pool, name, email, password).await {
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

async fn seed_customers(_pool: &Pool<Postgres>) -> AppResult<()> {
    Ok(())
}
