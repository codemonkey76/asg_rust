use core::fmt;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    HashError(argon2::password_hash::Error),
    DatabaseError(sqlx::Error),
    JsonWebTokenError(jsonwebtoken::errors::Error),
    SystemTimeError(std::time::SystemTimeError),
    UserNotFound(String),
    Base64DecodeError(base64::DecodeError),
    InvalidCredentials,
    InvalidPasswordHash,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::HashError(err) => write!(f, "Hashing Error: {}", err),
            AppError::DatabaseError(err) => write!(f, "Database Error: {}", err),
            AppError::JsonWebTokenError(err) => write!(f, "JSON Web Token Error: {}", err),
            AppError::SystemTimeError(err) => write!(f, "System Time Error: {}", err),
            AppError::UserNotFound(email) => write!(f, "User not found: {}", email),
            AppError::Base64DecodeError(err) => write!(f, "Base64 decode error: {}", err),
            AppError::InvalidCredentials => write!(f, "Invalid credentials"),
            AppError::InvalidPasswordHash => write!(f, "Invalid password hash"),
        }
    }
}

impl From<base64::DecodeError> for AppError {
    fn from(err: base64::DecodeError) -> Self {
        AppError::Base64DecodeError(err)
    }
}

impl From<std::time::SystemTimeError> for AppError {
    fn from(err: std::time::SystemTimeError) -> Self {
        AppError::SystemTimeError(err)
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AppError::JsonWebTokenError(err)
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
