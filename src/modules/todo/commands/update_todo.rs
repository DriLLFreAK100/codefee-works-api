use crate::{
    generated::schema::todos,
    modules::todo::models::{Todo, UpdateTodoRequest},
    utils::{db::*, http::*},
};
use actix_web::{put, web, Responder};
use diesel::prelude::*;

#[put("/{id}")]
pub async fn execute(
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
