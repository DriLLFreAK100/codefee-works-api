use crate::app::{
    db::*,
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
    let id = path.into_inner();
    let req_body = &req_body.into_inner();

    db_pool.get().run(|con| {
        diesel::update(todos::table.find(id))
            .set(req_body)
            .get_result::<Todo>(con)
            .into_res(String::from("Error updating todo"))
    })
}
