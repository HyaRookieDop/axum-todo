use std::{fmt::Display};
use axum::{response::IntoResponse, Json};
use crate::Response;

#[derive(Debug)]
pub enum AppErrorType {
    OK,
    DbType,
    NotFound,
}
#[derive(Debug)]
pub struct AppError {
    message: Option<String>,
    cause: Option<String>,
    error_type: AppErrorType,
}

impl AppError {
    fn code(&self) -> i32 {
        match self.error_type {
            AppErrorType::OK => 0,
            AppErrorType::DbType => 1,
            AppErrorType::NotFound => 2,
        }
    }
    /// 从上级错误中创建应用错误
    fn from_err(err: impl ToString, error_type: AppErrorType) -> Self {
        Self {
            message: None,
            cause: Some(err.to_string()),
            error_type,
        }
    }

    /// 从字符串创建应用错误
    fn from_str(msg: &str, error_type: AppErrorType) -> Self {
        Self {
            message: Some(msg.to_string()),
            cause: None,
            error_type,
        }
    }

    pub fn db_error(err: impl ToString) -> Self {
        Self::from_err(err, AppErrorType::DbType)
    }

    pub fn not_found() -> Self {
        Self::from_str("不存在的记录", AppErrorType::NotFound)
    }
}

impl From<deadpool_postgres::PoolError> for AppError {
    fn from(err: deadpool_postgres::PoolError) -> Self {
        Self::db_error(err)
    }
}

impl From<tokio_postgres::Error> for AppError {
    fn from(err: tokio_postgres::Error) -> Self {
        Self::db_error(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let code = (&self).code();
        let msg = match self.message {
            Some(msg) => msg,
            None => "有错误发生".to_string(),
        };
        let res: Response<()> = Response::err(code, msg);
        Json(res).into_response()
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}