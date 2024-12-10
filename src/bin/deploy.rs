use clap::{Parser, Subcommand};
use dotenv::dotenv;
use sqlx::{postgres::PgConnectOptions, PgPool};
use std::{env, fmt};

const SCHEMA: &str = "public";

#[derive(Parser)]
#[command(name = "Deploy Script")]
#[command(about = "Manage database deployment", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create,
    Delete,
}

#[tokio::main]
async fn main() -> DeployResult<()> {
    dotenv().ok();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Create => create_database().await?,
        Commands::Delete => delete_database().await?,
    }
    Ok(())
}

async fn grant_schema_privileges(schema: &str, db_user: &str, pool: &PgPool) -> DeployResult<()> {
    let queries = vec![
        format!("GRANT USAGE, CREATE ON SCHEMA {} TO {}", schema, db_user),
        format!(
            "ALTER DEFAULT PRIVILEGES IN SCHEMA {} GRANT ALL PRIVILEGES ON TABLES TO {}",
            schema, db_user
        ),
        format!(
            "ALTER DEFAULT PRIVILEGES IN SCHEMA {} GRANT ALL PRIVILEGES ON SEQUENCES TO {}",
            schema, db_user
        ),
    ];
    let current_info: (String, String) = sqlx::query_as("SELECT current_database(), current_user")
        .fetch_one(pool)
        .await?;
    println!(
        "Connected to database: {}, as user: {}",
        current_info.0, current_info.1
    );

    for query in queries {
        println!("Executing SQL: {}", &query);
        sqlx::query(&query).execute(pool).await?;
    }

    println!(
        "Schema-level privileges granted to user '{}' on schema '{}'",
        db_user, schema
    );

    Ok(())
}

async fn create_database() -> DeployResult<()> {
    let pool = get_root_pool().await?;

    // Load environment variables
    let db_name =
        env::var("DB_NAME").map_err(|_| DeployError::EnvVarError("DB_NAME not set".to_string()))?;
    let db_user =
        env::var("DB_USER").map_err(|_| DeployError::EnvVarError("DB_USER not set".to_string()))?;
    let db_password = env::var("DB_PASSWORD")
        .map_err(|_| DeployError::EnvVarError("DB_PASSWORD not set".to_string()))?;

    let create_db_query = format!("SELECT 1 FROM pg_database WHERE datname = '{}'", db_name);
    let exists: Option<i32> = sqlx::query_scalar(&create_db_query)
        .fetch_optional(&pool)
        .await?;
    if exists.is_none() {
        sqlx::query(&format!("CREATE DATABASE {}", db_name))
            .execute(&pool)
            .await?;
        println!("Database '{}' created", db_name);
    } else {
        println!("Database '{}' already exists", db_name);
    }

    // Create the user and grant privileges
    let create_user_query = format!("SELECT 1 FROM pg_roles WHERE rolname = '{}'", db_user);
    let user_exists: Option<i32> = sqlx::query_scalar(&create_user_query)
        .fetch_optional(&pool)
        .await?;
    if user_exists.is_none() {
        sqlx::query(&format!(
            "CREATE ROLE {} WITH LOGIN PASSWORD '{}'",
            db_user, db_password
        ))
        .execute(&pool)
        .await?;
        println!("User '{}' created", db_user);
    } else {
        println!("User '{}' already exists", db_user);
    }

    sqlx::query(&format!(
        "GRANT ALL PRIVILEGES ON DATABASE {} TO {}",
        db_name, db_user
    ))
    .execute(&pool)
    .await?;

    println!(
        "Database-level privileges granted to user '{}' on database '{}'",
        db_user, db_name,
    );
    let db_pool = PgPool::connect_with(
        sqlx::postgres::PgConnectOptions::new()
            .host(&env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()))
            .port(
                env::var("DB_PORT")
                    .unwrap_or_else(|_| "5432".to_string())
                    .parse()
                    .unwrap(),
            )
            .username(&env::var("DB_ROOT_USER").unwrap_or_else(|_| "postgres".to_string()))
            .password(&env::var("DB_ROOT_PASSWORD").unwrap_or_else(|_| "".to_string()))
            .database(&db_name),
    )
    .await?;
    println!("Connected to the '{}' database", db_name);
    grant_schema_privileges(SCHEMA, &db_user, &db_pool).await?;

    Ok(())
}

async fn delete_database() -> DeployResult<()> {
    let pool = get_root_pool().await?;

    // Load environment variables
    let db_name =
        env::var("DB_NAME").map_err(|_| DeployError::EnvVarError("DB_NAME not set".to_string()))?;
    let db_user =
        env::var("DB_USER").map_err(|_| DeployError::EnvVarError("DB_USER not set".to_string()))?;

    sqlx::query(&format!(
        "REVOKE ALL PRIVILEGES ON SCHEMA {} FROM {}",
        SCHEMA, db_user
    ))
    .execute(&pool)
    .await?;
    sqlx::query(&format!(
        "REVOKE ALL PRIVILEGES ON ALL TABLES IN SCHEMA {} FROM {}",
        SCHEMA, db_user
    ))
    .execute(&pool)
    .await?;
    sqlx::query(&format!(
        "REVOKE ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA {} FROM {}",
        SCHEMA, db_user
    ))
    .execute(&pool)
    .await?;
    sqlx::query(&format!(
        "ALTER DEFAULT PRIVILEGES IN SCHEMA {} REVOKE ALL ON TABLES FROM {}",
        SCHEMA, db_user
    ))
    .execute(&pool)
    .await?;
    sqlx::query(&format!(
        "ALTER DEFAULT PRIVILEGES IN SCHEMA {} REVOKE ALL ON SEQUENCES FROM {}",
        SCHEMA, db_user
    ))
    .execute(&pool)
    .await?;

    println!(
        "Revoked schema-level privileges for user '{}' on schema '{}'",
        db_user, SCHEMA
    );

    // Drop the database
    sqlx::query(&format!("DROP DATABASE IF EXISTS {}", db_name))
        .execute(&pool)
        .await?;
    println!("Database '{}' dropped", db_name);

    // Drop the user
    sqlx::query(&format!("DROP ROLE IF EXISTS {}", db_user))
        .execute(&pool)
        .await?;
    println!("User '{}' dropped", db_user);

    Ok(())
}

async fn get_root_pool() -> DeployResult<PgPool> {
    let db_host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
    let db_port: u16 = env::var("DB_PORT")
        .unwrap_or_else(|_| "5432".to_string())
        .parse()
        .map_err(|_| DeployError::EnvVarError("DB_PORT is invalid".to_string()))?;
    let db_root_user = env::var("DB_ROOT_USER").unwrap_or_else(|_| "postgres".to_string());
    let db_root_password = env::var("DB_ROOT_PASSWORD").unwrap_or_else(|_| "".to_string());

    let options = PgConnectOptions::new()
        .host(&db_host)
        .port(db_port)
        .username(&db_root_user)
        .password(&db_root_password);

    PgPool::connect_with(options)
        .await
        .map_err(DeployError::ConnectionError)
}

pub type DeployResult<T> = Result<T, DeployError>;

#[derive(Debug)]
pub enum DeployError {
    EnvVarError(String),
    ConnectionError(sqlx::Error),
}

impl fmt::Display for DeployError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeployError::EnvVarError(msg) => write!(f, "Environment variable error: {}", msg),
            DeployError::ConnectionError(err) => write!(f, "Database connection error: {}", err),
        }
    }
}

impl std::error::Error for DeployError {}

impl From<sqlx::Error> for DeployError {
    fn from(err: sqlx::Error) -> Self {
        DeployError::ConnectionError(err)
    }
}
