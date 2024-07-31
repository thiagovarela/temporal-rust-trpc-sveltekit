

#[derive(Debug)]
pub enum AppError {
    ArgonError(argon2::Error),
    PasswordError(argon2::password_hash::Error),
    QueryError(sqlx::Error),   
    ActivityError(temporal_sdk::ActivityError), 
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        AppError::QueryError(error.into())
    }
}

impl From<temporal_sdk::ActivityError> for AppError {
    fn from(error: temporal_sdk::ActivityError) -> Self {
        AppError::ActivityError(error.into())
    }
}

impl From<argon2::Error> for AppError {
    fn from(error: argon2::Error) -> Self {
        AppError::ArgonError(error.into())
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(error: argon2::password_hash::Error) -> Self {
        AppError::PasswordError(error.into())
    }
}