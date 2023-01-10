use super::commands::{create_todo, delete_todo, get_todo, get_todos, link_todos, update_todo};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todo")
            .service(create_todo::create_todo)
            .service(get_todo::get_todo)
            .service(get_todos::get_todos)
            .service(link_todos::link_todos)
            .service(update_todo::update_todo)
            .service(delete_todo::delete_todo),
    );
}
