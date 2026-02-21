use std::str::FromStr;

use super::errors::AppError;

pub fn get_env_vars<T>(key: &str) -> Result<T, AppError>
where
    T: FromStr,
{
    let vars = std::env::var(key)
        .map_err(|_| AppError::NotFound(format!("Environment variable {key} not Found")))?;

    Ok(vars
        .parse::<T>()
        .map_err(|_| AppError::BadRequest("An error occured while parsing".to_string())))?
}
