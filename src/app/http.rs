use super::error::AppError;
use actix_web::HttpResponse;
use serde::Serialize;

pub trait DbToHttp {
    fn into_res(&self, expect: String) -> Result<HttpResponse, AppError>;
}

impl<D> DbToHttp for Result<D, diesel::result::Error>
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
