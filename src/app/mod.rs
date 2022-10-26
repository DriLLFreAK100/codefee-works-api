use actix_web::web;
mod todo;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todo")
            .service(todo::commands::create_todo::execute)
            .service(todo::commands::get_todo::execute)
            .service(todo::commands::update_todo::execute)
            .service(todo::commands::delete_todo::execute),
    );
}
