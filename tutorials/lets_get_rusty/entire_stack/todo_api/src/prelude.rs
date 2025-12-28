pub use crate::error::Error;
pub use crate::error::ApiError;

pub type Result<T> = std::result::Result<T, Error>;
pub type ApiResult<T> = std::result::Result<T, ApiError>;
