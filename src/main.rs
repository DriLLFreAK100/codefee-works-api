use actix_web::{App, HttpServer};
mod app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    app::db::configure_db();

    HttpServer::new(|| App::new().configure(app::configure_routes))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
