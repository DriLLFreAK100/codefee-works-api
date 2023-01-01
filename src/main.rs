#[macro_use]
extern crate serde;
#[macro_use]
extern crate diesel;

use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;
use utils::db;

// Register custom mods
mod generated;
mod modules;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Core API Starting...");

    dotenv().ok().expect("Env init error");
    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");
    let pool = Data::new(db::get_connection_pool());
    println!("Configurations loaded successfully");

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .configure(modules::todo::routes::configure)
    })
    .bind((host, port.parse::<u16>().unwrap()))?
    .run()
    .await
}
