use std::{env, fmt};

use clap::{Parser, Subcommand};
use sqlx::{Pool, Postgres};

#[tokio::main]
async fn main() -> SeedResult<()> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Pool::<Postgres>::connect(&database_url).await?;

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::All) => {
            seed_users(&pool).await?;
            seed_customers(&pool).await?;
        }
        Some(Commands::Users) => {}
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

async fn seed_users(pool: &Pool<Postgres>) -> SeedResult<()> {
    Ok(())
}

async fn seed_customers(pool: &Pool<Postgres>) -> SeedResult<()> {
    Ok(())
}

pub type SeedResult<T> = Result<T, SeedError>;

#[derive(Debug)]
pub enum SeedError {
    ConnectionError(sqlx::Error),
}

impl fmt::Display for SeedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SeedError::ConnectionError(err) => write!(f, "Database connection error: {}", err),
        }
    }
}

impl std::error::Error for SeedError {}

impl From<sqlx::Error> for SeedError {
    fn from(err: sqlx::Error) -> Self {
        SeedError::ConnectionError(err)
    }
}
