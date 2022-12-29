use std::error::Error;

/// Represents custom Crawler errors.
#[derive(Debug)]
pub enum ApiError {
    {% if redis != "none" %}
    RedisConnectionPoolError(Box<dyn Error + Send + Sync>),
    RedisConnectionError(Box<dyn Error + Send + Sync>),
    _RedisCommandError(Box<dyn Error + Send + Sync>),
    {% endif %}
    PostgresConnectionPoolError(Box<dyn Error + Send + Sync>),
    _PostgresConnectionError(Box<dyn Error + Send + Sync>),
    _PostgresSQLError(Box<dyn Error + Send + Sync>),
    SetGlobalTracerError(Box<dyn Error + Send + Sync>),
}
impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            {% if redis != "none" %}
            ApiError::RedisConnectionPoolError(e) => f.write_fmt(format_args!("API4: {}", e)),
            ApiError::RedisConnectionError(_) => f.write_str("API5"),
            ApiError::_RedisCommandError(e) => f.write_fmt(format_args!("API6: {}", e)),
            {% endif %}
            ApiError::PostgresConnectionPoolError(e) => f.write_fmt(format_args!("API7: {}", e)),
            ApiError::_PostgresConnectionError(e) => f.write_fmt(format_args!("API8: {}", e)),
            ApiError::_PostgresSQLError(e) => f.write_fmt(format_args!("API_9: {}", e)),
            ApiError::SetGlobalTracerError(e) => f.write_fmt(format_args!("API_10: {}", e)),
        }
    }
}
