use crate::{
    generated::schema::todos,
    modules::todo::models::{Todo, UpdateTodoRequest},
    utils::{db::*, http::*},
};
use actix_web::{put, web, Responder};
use diesel::prelude::*;

/// Update todo by ID
#[utoipa::path(
    path = "/todo/{id}",
    responses(
        (status = 200, description = "Updated todo successfully", body = Todo)
    ),
    tag="todo"
)]
#[put("/{id}")]
pub async fn update_todo(
    path: web::Path<i32>,
    req_body: web::Json<UpdateTodoRequest>,
    db_pool: web::Data<PostgresPool>,
) -> impl Responder {
    let id = path.into_inner();
    let req_body = &req_body.into_inner();

    db_pool.run(|con| {
        diesel::update(todos::table.find(id))
            .set(req_body)
            .get_result::<Todo>(con)
            .into_res("Error updating todo")
    })
}
