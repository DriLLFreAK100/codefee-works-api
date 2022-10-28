use actix_web::{App, HttpServer};
mod app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    app::db::configure();

    HttpServer::new(|| App::new().configure(app::routes::configure))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
