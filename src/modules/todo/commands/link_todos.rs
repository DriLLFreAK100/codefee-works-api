use crate::{
    generated::schema::todos_relations::{self, child_todo_id, parent_todo_id, relationship_type},
    modules::todo::models::LinkTodosRequest,
    utils::{db::*, http::*},
};
use actix_web::{put, web, Responder};
use diesel::prelude::*;

/// Link todos to a todo
#[utoipa::path(
  path = "/todo/link/{id}",
  request_body = LinkTodosRequest,
  responses(
      (status = 200, description = "Todos linked successfully", body = bool)
  ),
  tag="todo"
)]
#[put("/link/{id}")]
pub async fn link_todos(
    db_pool: web::Data<PostgresPool>,
    path: web::Path<i32>,
    req_body: web::Json<LinkTodosRequest>,
) -> impl Responder {
    let id = path.into_inner();
    let req_body = &req_body.into_inner();

    db_pool.run(|con| {
        diesel::delete(todos_relations::table.filter(parent_todo_id.eq(id)))
            .execute(con)
            .expect("Error deleting todo links");

        let mut data = vec![];

        for todo_id in req_body.todo_ids.iter() {
            data.push((
                parent_todo_id.eq(id),
                child_todo_id.eq(*todo_id),
                relationship_type.eq(req_body.relationship_type),
            ));
        }

        diesel::insert_into(todos_relations::table)
            .values(&data)
            .execute(con)
            .into_affected_res("Error linking todos")
    })
}
