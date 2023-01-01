use super::error::AppError;
use actix_web::HttpResponse;
use diesel::QueryResult;
use serde::Serialize;

pub trait JsonResponse {
    /// Convert data to an HttpResponse result
    fn into_res(&self, expect: &str) -> Result<HttpResponse, AppError>;
}

impl<D> JsonResponse for Result<D, diesel::result::Error>
where
    D: Serialize,
{
    fn into_res(&self, expect: &str) -> Result<HttpResponse, AppError> {
        match self {
            Ok(data) => Ok(HttpResponse::Ok().json(data)),
            Err(e) => {
                println!("[Error]: {:?}", e);

                Err(AppError::CustomError {
                    message: expect.to_owned(),
                })
            }
        }
    }
}

pub trait RowAffectedResponse {
    /// Convert affected rows result (e.g. via `execute`) to an HttpResponse result
    fn into_affected_res(&self, expect: &str) -> Result<HttpResponse, AppError>;
}

impl RowAffectedResponse for QueryResult<usize> {
    fn into_affected_res(&self, expect: &str) -> Result<HttpResponse, AppError> {
        match self {
            Ok(data) => {
                if (*data as usize) > 0 {
                    return Ok(HttpResponse::Ok().finish());
                }

                return Err(AppError::CustomError {
                    message: String::from("Operation does not affect any row"),
                });
            }
            Err(e) => {
                println!("[Error]: {:?}", e);

                Err(AppError::CustomError {
                    message: expect.to_owned(),
                })
            }
        }
    }
}
