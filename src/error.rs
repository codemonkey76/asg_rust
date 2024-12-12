use core::fmt;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    HashError(argon2::password_hash::Error),
    DatabaseError(sqlx::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::HashError(err) => write!(f, "Hashing Error: {}", err),
            AppError::DatabaseError(err) => write!(f, "Database Error: {}", err),
        }
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(err: argon2::password_hash::Error) -> Self {
        AppError::HashError(err)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err)
    }
}
