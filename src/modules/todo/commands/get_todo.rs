use crate::{
    generated::schema::todos,
    modules::todo::models::Todo,
    utils::{db::*, http::*},
};
use actix_web::{get, web, Responder};
use diesel::prelude::*;

/// Get todo by ID
#[utoipa::path(
  path = "/todo/{id}",
  responses(
      (status = 200, description = "Get todo successfully", body = Todo)
  ),
  tag="todo"
)]
#[get("/{id}")]
pub async fn get_todo(path: web::Path<i32>, db_pool: web::Data<PostgresPool>) -> impl Responder {
    let id = path.into_inner();

    db_pool.run(|con| {
        todos::table
            .find(id)
            .first::<Todo>(con)
            .into_res("Error finding todo")
    })
}
