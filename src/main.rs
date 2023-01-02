#[macro_use]
extern crate serde;
#[macro_use]
extern crate diesel;

use actix_web::web::Data;
use actix_web::{App, HttpServer};
use configs::{cors::with_cors, open_api::with_swagger};
use dotenv::dotenv;
use std::env;
use utils::db;
use utils::env::is_dev;

// Register custom mods
mod configs;
mod generated;
mod modules;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Core API Starting...");

    // Load environment variables
    dotenv().ok().expect("Env init error");
    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");
    let pool = Data::new(db::get_connection_pool());
    println!("Configurations loaded successfully");

    // Start actix server
    HttpServer::new(move || {
        let app = App::new()
            .wrap(with_cors())
            .app_data(pool.clone())
            .configure(modules::todo::routes::configure);

        match is_dev() {
            true => app.service(with_swagger()),
            false => app,
        }
    })
    .bind((host, port.parse::<u16>().unwrap()))?
    .run()
    .await
}
