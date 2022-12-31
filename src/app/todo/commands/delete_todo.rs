use crate::app::{
    db::PostgresPool,
    schema::todos::dsl::{id, todos},
};
use actix_web::{delete, web, HttpResponse, Responder};
use diesel::prelude::*;

#[delete("/{id}")]
pub async fn execute(path: web::Path<i32>, db_pool: web::Data<PostgresPool>) -> impl Responder {
    match db_pool.get() {
        Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
        con => {
            let target_id = path.into_inner();

            diesel::delete(todos.filter(id.eq(target_id)))
                .execute(&mut con.unwrap())
                .expect("Error deleting todo");

            HttpResponse::Ok().finish()
        }
    }
}
