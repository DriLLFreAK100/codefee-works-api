use actix_web::{get, HttpResponse, Responder};

#[get("")]
pub async fn execute() -> impl Responder {
    HttpResponse::Ok().body("Todo")
}
