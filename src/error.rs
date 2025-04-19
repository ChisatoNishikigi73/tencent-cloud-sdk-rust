use thiserror::Error;

/// 腾讯云API SDK错误类型
#[derive(Error, Debug)]
pub enum Error {
    /// API请求错误
    #[error("API请求错误: {0}")]
    RequestError(#[from] reqwest::Error),

    /// 服务器返回错误
    #[error("腾讯云API错误: {code} - {message}")]
    ApiError {
        code: String,
        message: String,
    },

    /// JSON解析错误
    #[error("JSON解析错误: {0}")]
    SerdeError(#[from] serde_json::Error),
    
    /// HTTP头解析错误
    #[error("HTTP头解析错误: {0}")]
    HeaderError(String),

    /// 其他错误
    #[error("其他错误: {0}")]
    Other(String),
}

/// 结果类型别名
pub type Result<T> = std::result::Result<T, Error>;

impl From<http::header::InvalidHeaderName> for Error {
    fn from(err: http::header::InvalidHeaderName) -> Self {
        Error::HeaderError(format!("无效的头名称: {}", err))
    }
}

impl From<http::header::InvalidHeaderValue> for Error {
    fn from(err: http::header::InvalidHeaderValue) -> Self {
        Error::HeaderError(format!("无效的头值: {}", err))
    }
} 