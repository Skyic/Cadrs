use thiserror::Error;
use std::fmt;

#[derive(Debug, Error)]
pub enum CADError {
    #[error("几何错误: {0}")]
    GeometryError(String),
    
    #[error("文件IO错误: {0}")]
    IOError(String),
    
    #[error("解析错误: {0}")]
    ParseError(String),
    
    #[error("验证错误: {0}")]
    ValidationError(String),
    
    #[error("未找到: {0}")]
    NotFound(String),
    
    #[error("无效操作: {0}")]
    InvalidOperation(String),
    
    #[error("转换错误: {0}")]
    ConversionError(String),
    
    #[error("版本不兼容: {0}")]
    VersionError(String),
    
    #[error("插件错误: {0}")]
    PluginError(String),
}

pub type CADResult<T> = Result<T, CADError>;

impl From<std::io::Error> for CADError {
    fn from(error: std::io::Error) -> Self {
        CADError::IOError(error.to_string())
    }
}

impl From<std::fmt::Error> for CADError {
    fn from(error: std::fmt::Error) -> Self {
        CADError::GeometryError(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = CADError::GeometryError("Invalid point".to_string());
        assert_eq!(format!("{}", error), "几何错误: Invalid point");
    }

    #[test]
    fn test_result_type() {
        let result: CADResult<f64> = Ok(42.0);
        assert!(result.is_ok());
        
        let result: CADResult<f64> = Err(CADError::NotFound("Point".to_string()));
        assert!(result.is_err());
    }
}
