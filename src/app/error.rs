use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum AppError {
    #[display(fmt = "An internal server error occurred.")]
    ServerError,

    #[display(fmt = "{}", message)]
    CustomError { message: String },
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            AppError::ServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::CustomError { .. } => StatusCode::BAD_REQUEST,
        }
    }
}
