use crate::app::{
    db::PostgresPool,
    error::AppError,
    http::*,
    schema::todos,
    todo::models::{Todo, UpdateTodoRequest},
};
use actix_web::{put, web, HttpResponse};
use diesel::prelude::*;

#[put("/{id}")]
pub async fn execute(
    path: web::Path<i32>,
    req_body: web::Json<UpdateTodoRequest>,
    db_pool: web::Data<PostgresPool>,
) -> Result<HttpResponse, AppError> {
    match db_pool.get() {
        Ok(mut con) => {
            let id = path.into_inner();

            diesel::update(todos::table.find(id))
                .set(req_body.into_inner())
                .get_result::<Todo>(&mut con)
                .into_res(String::from("Error updating todo"))
        }
        _ => Err(AppError::ServerError),
    }
}
