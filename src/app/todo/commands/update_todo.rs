use actix_web::{put, web, HttpResponse, Responder};

#[put("/{id}")]
pub async fn execute(path: web::Path<(u32,)>, req_body: String) -> impl Responder {
    let (id,) = path.into_inner();
    HttpResponse::Ok().body(format!("Updating {} {}", id.to_string(), req_body))
}
