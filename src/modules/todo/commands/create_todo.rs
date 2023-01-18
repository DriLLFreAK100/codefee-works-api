use crate::{
    generated::schema::todos,
    modules::todo::models::{Todo, UpdateTodoRequest},
    utils::{db::*, http::*},
};
use actix_web::{post, web, Responder};
use diesel::prelude::*;

/// Create todo
#[utoipa::path(
    path = "/todo",
    request_body = UpdateTodoRequest,
    responses(
        (status = 200, description = "Created a todo item successfully", body = Todo)
    ),
    tag="todo"
)]
#[post("")]
pub async fn create_todo(
    db_pool: web::Data<PostgresPool>,
    req_body: web::Json<UpdateTodoRequest>,
) -> impl Responder {
    let req_body = &req_body.into_inner();

    db_pool.run(|con| {
        diesel::insert_into(todos::table)
            .values(req_body)
            .get_result::<Todo>(con)
            .into_res("Error creating todo")
    })
}
