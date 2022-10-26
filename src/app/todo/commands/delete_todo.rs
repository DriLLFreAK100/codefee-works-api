use actix_web::{delete, web, HttpResponse, Responder};

#[delete("/{id}")]
pub async fn execute(path: web::Path<(u32,)>, req_body: String) -> impl Responder {
    let (id,) = path.into_inner();
    HttpResponse::Ok().body(format!("Deleting {} {}", id.to_string(), req_body))
}
