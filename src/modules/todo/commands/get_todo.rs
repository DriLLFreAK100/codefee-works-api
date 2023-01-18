use crate::{
    generated::schema::todos,
    modules::todo::models::{Todo, TodoRelation},
    utils::db::*,
};
use actix_web::{get, web, HttpResponse, Responder};
use diesel::prelude::*;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RelatedTodoResponse {
    pub child_todo_id: Option<i32>,
    pub relationship_type: i16,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TodoDetailsResponse {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: i16,
    pub tags: Option<Vec<Option<String>>>,
    pub related_todos: Vec<RelatedTodoResponse>,
}

/// Map a todo and its related todos into a dto
fn map_todo_details(todo: Todo, todo_links: Vec<TodoRelation>) -> TodoDetailsResponse {
    TodoDetailsResponse {
        id: todo.id,
        description: todo.description,
        status: todo.status,
        title: todo.title,
        tags: todo.tags,
        related_todos: todo_links
            .iter()
            .map(|x| RelatedTodoResponse {
                child_todo_id: x.child_todo_id,
                relationship_type: x.relationship_type,
            })
            .collect(),
    }
}

/// Get todo by ID
#[utoipa::path(
  path = "/todo/{id}",
  responses(
      (status = 200, description = "Get todo successfully", body = TodoDetailsResponse)
  ),
  tag="todo"
)]
#[get("/{id}")]
pub async fn get_todo(path: web::Path<i32>, db_pool: web::Data<PostgresPool>) -> impl Responder {
    let id = path.into_inner();

    db_pool.run(|con| {
        let todo: Todo = todos::table
            .find(id)
            .first(con)
            .expect("Error fetching todo");

        let todo_links: Vec<TodoRelation> = TodoRelation::belonging_to(&todo)
            .load(con)
            .expect("Error fetching todo links");

        Ok(HttpResponse::Ok().json(map_todo_details(todo, todo_links)))
    })
}
