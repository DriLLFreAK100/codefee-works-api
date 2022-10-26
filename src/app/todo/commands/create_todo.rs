use actix_web::{post, HttpResponse, Responder};

#[post("")]
pub async fn execute(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
