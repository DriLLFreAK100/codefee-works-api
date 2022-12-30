#[macro_use]
extern crate serde;
#[macro_use]
extern crate diesel;

use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

mod app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok().expect("Env init error");
    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");

    let pool = Data::new(crate::app::db::get_connection_pool());

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .configure(app::routes::configure)
    })
    .bind((host, port.parse::<u16>().unwrap()))?
    .run()
    .await
}
