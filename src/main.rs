use actix_web::{App, HttpServer};
use app::schema::todos::dsl::*;
use app::todo::models::*;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

mod app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");

    let connection = &mut app::db::establish_connection();
    let results = todos
        .limit(5)
        .load::<Todo>(connection)
        .expect("Error loading todos");

    println!("Displaying {} todos", results.len());
    for todo in results {
        println!("{}", todo.title);
        println!("-----------\n");
        println!("{}", format!("{:?}", todo.description));
    }

    HttpServer::new(move || App::new().configure(app::routes::configure))
        .bind((host, port.parse::<u16>().unwrap()))?
        .run()
        .await
}
