use crate::app::{
    db::PostgresPool,
    schema::todos,
    todo::models::{Todo, UpdateTodoRequest},
};
use actix_web::{put, web, HttpResponse, Responder};
use diesel::prelude::*;

#[put("/{id}")]
pub async fn execute(
    path: web::Path<i32>,
    req_body: web::Json<UpdateTodoRequest>,
    db_pool: web::Data<PostgresPool>,
) -> impl Responder {
    match db_pool.get() {
        Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
        con => {
            let id = path.into_inner();

            let todo: Todo = diesel::update(todos::table.find(id))
                .set(req_body.into_inner())
                .get_result(&mut con.unwrap())
                .expect("Error updating todo");

            HttpResponse::Ok().json(todo)
        }
    }
}
