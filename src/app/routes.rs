use crate::app::todo::commands::{create_todo, delete_todo, get_todo, update_todo};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todo")
            .service(create_todo::execute)
            .service(get_todo::execute)
            .service(update_todo::execute)
            .service(delete_todo::execute),
    );
}
