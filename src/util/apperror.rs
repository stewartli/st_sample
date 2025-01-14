use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

// don't crash server
// pub async fn custom_err() -> Result<(), StatusCode> {
//     Err(StatusCode::FORBIDDEN)
// }

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error_msg: String,
}

pub struct AppError {
    status_code: StatusCode,
    err_msg: String,
}

#[allow(dead_code)]
impl AppError {
    pub fn new(status_code: StatusCode, err_msg: impl Into<String>) -> Self {
        Self {
            status_code,
            err_msg: err_msg.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            self.status_code,
            Json(ErrorResponse {
                error_msg: self.err_msg,
            }),
        )
            .into_response()
    }
}
