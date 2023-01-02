use super::commands::{create_todo, delete_todo, get_todos, update_todo};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todo")
            .service(create_todo::execute)
            .service(get_todos::execute)
            .service(update_todo::execute)
            .service(delete_todo::execute),
    );
}
