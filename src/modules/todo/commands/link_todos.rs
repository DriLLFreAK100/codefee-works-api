use std::collections::HashSet;

use crate::{
    generated::schema::todos_relations::{self, child_todo_id, parent_todo_id, relationship_type},
    modules::todo::models::TodoRelation,
    utils::db::*,
};
use actix_web::{put, web, HttpResponse, Responder};
use diesel::prelude::*;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LinkTodosRequest {
    pub todo_ids: Vec<i32>,
    pub relationship_type: i16,
}

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
        let mut existing_link_ids: HashSet<i32> = todos_relations::table
            .filter(parent_todo_id.eq(id))
            .load::<TodoRelation>(con)
            .expect("Error fetching existing links")
            .into_iter()
            .map(|e| e.child_todo_id.unwrap())
            .collect();

        let mut to_adds = vec![];

        for id in req_body.todo_ids.clone() {
            if !existing_link_ids.contains(&id) {
                to_adds.push(id);
                continue;
            }

            // Remove id to persist. Leftovers will be deleted
            existing_link_ids.remove(&id);
        }

        // Delete unlink todos
        diesel::delete(todos_relations::table.filter(child_todo_id.eq_any(existing_link_ids)))
            .execute(con)
            .expect("Error deleting todo links");

        // Insert new links
        if !to_adds.is_empty() {
            let mut data = vec![];
            for todo_id in to_adds {
                data.push((
                    parent_todo_id.eq(id),
                    child_todo_id.eq(todo_id),
                    relationship_type.eq(req_body.relationship_type),
                ));
            }

            diesel::insert_into(todos_relations::table)
                .values(&data)
                .execute(con)
                .expect("Error linking todos");
        }

        Ok(HttpResponse::Ok())
    })
}
