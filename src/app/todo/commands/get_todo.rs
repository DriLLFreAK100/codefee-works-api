use crate::app::{db::*, http::*, schema::todos, todo::models::Todo};
use crate::diesel::RunQueryDsl;
use actix_web::{get, web, Responder};

#[get("")]
pub async fn execute(db_pool: web::Data<PostgresPool>) -> impl Responder {
    db_pool.get().run(|con| {
        todos::table
            .load::<Todo>(con)
            .into_res("Error reading todos")
    })
}
