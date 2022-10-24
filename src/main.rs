use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn get_todo() -> impl Responder {
    HttpResponse::Ok().body("Todo")
}

#[post("/")]
async fn create_todo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[put("/{id}")]
async fn update_todo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[delete("/{id}")]
async fn delete_todo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/todo")
                .service(create_todo)
                .service(get_todo)
                .service(update_todo)
                .service(delete_todo),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
