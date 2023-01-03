#[macro_use]
extern crate serde;
#[macro_use]
extern crate diesel;

use actix_web::{web::Data, App, HttpServer};
use configs::{
    cors::with_cors,
    logger::{end_telemetry, init_telemetry, with_logger},
    open_api::with_swagger,
};
use utils::db::get_connection_pool;
use utils::env::{get_host_port, init_env, is_dev};

// Register custom mods
mod configs;
mod generated;
mod modules;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Core API Starting...");

    init_env();
    init_telemetry();
    println!("Configurations loaded successfully");

    let pool = Data::new(get_connection_pool());

    // Start actix server
    HttpServer::new(move || {
        let app = App::new()
            .wrap(with_cors())
            .wrap(with_logger())
            .app_data(pool.clone())
            .configure(modules::todo::routes::configure);

        match is_dev() {
            true => app.service(with_swagger()),
            false => app,
        }
    })
    .bind(get_host_port())?
    .run()
    .await?;

    end_telemetry();

    Ok(())
}
