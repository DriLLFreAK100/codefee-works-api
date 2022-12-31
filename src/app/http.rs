use super::error::AppError;
use actix_web::HttpResponse;
use serde::Serialize;

pub trait HttpResponseMapper {
    /// Convert to an actual HttpResponse result
    fn into_res(&self, expect: String) -> Result<HttpResponse, AppError>;
}

impl<D> HttpResponseMapper for Result<D, diesel::result::Error>
where
    D: Serialize,
{
    fn into_res(&self, expect: String) -> Result<HttpResponse, AppError> {
        match self {
            Ok(data) => Ok(HttpResponse::Ok().json(data)),
            _ => Err(AppError::CustomError { message: expect }),
        }
    }
}
