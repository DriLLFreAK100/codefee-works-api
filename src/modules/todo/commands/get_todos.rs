use crate::{
    generated::schema::todos,
    modules::todo::models::Todo,
    utils::{db::*, http::*},
};
use actix_web::{get, web, Responder};
use diesel::prelude::*;

/// Get all todos
#[utoipa::path(
    responses(
        (status = 200, description = "Get all todos successfully", body = [Todo])
    ),
    tag="todo"
)]
#[get("")]
pub async fn execute(db_pool: web::Data<PostgresPool>) -> impl Responder {
    db_pool.run(|con| {
        todos::table
            .load::<Todo>(con)
            .into_res("Error reading todos")
    })
}
