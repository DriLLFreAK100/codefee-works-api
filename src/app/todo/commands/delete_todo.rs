use crate::app::{
    db::*,
    http::*,
    schema::todos::dsl::{id, todos},
};
use actix_web::{delete, web, Responder};
use diesel::prelude::*;

#[delete("/{id}")]
pub async fn execute(path: web::Path<i32>, db_pool: web::Data<PostgresPool>) -> impl Responder {
    let target_id = path.into_inner();

    db_pool.get().run(|con| {
        diesel::delete(todos.filter(id.eq(target_id)))
            .execute(con)
            .into_affected_res("Error deleting todo")
    })
}
