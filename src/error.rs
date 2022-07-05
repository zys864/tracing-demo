use axum::http::header;
use axum::{
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde::Serialize;
use tracing::warn;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("UnAuth")]
    UnAnuth,
    #[error(transparent)]
    InternalServerError(#[from] anyhow::Error),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    SqlError(#[from] sqlx::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        ErrMsg::from(self).into_response()
    }
}
#[derive(Debug, Serialize)]
struct ErrMsg {
    status_code: u16,
    msg: String,
}

impl ErrMsg {
    fn new(status_code: StatusCode, msg: String) -> Self {
        Self {
            status_code: status_code.as_u16(),
            msg,
        }
    }
}
impl core::fmt::Display for ErrMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}
impl IntoResponse for ErrMsg {
    fn into_response(self) -> axum::response::Response {
        if cfg!(debug) {
            warn!(error = %self.msg)
        }
        Response::builder()
            .header(header::CONTENT_TYPE, "APPLICATION/JSON")
            .status(self.status_code)
            .body(axum::body::boxed(self.msg))
            .unwrap()
    }
}
impl From<Error> for ErrMsg {
    fn from(error: Error) -> Self {
        match error {
            Error::UnAnuth => Self::new(StatusCode::UNAUTHORIZED, "UnAuth".to_string()),
            Error::InternalServerError(_) | Error::SqlError(_) => Self::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "InternalServerError".to_string(),
            ),
            Error::ValidationError(e) => Self::new(StatusCode::UNPROCESSABLE_ENTITY, e.to_string()),
        }
    }
}
