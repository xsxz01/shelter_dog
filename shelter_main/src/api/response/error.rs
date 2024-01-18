use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub struct AppError(anyhow::Error);

#[derive(Serialize)]
pub(crate) struct ErrorResponse {
    pub(crate) status: String,
    pub(crate) message: String
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                status: "error".to_string(),
                message: self.0.to_string(),
            }),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
    where
        E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
