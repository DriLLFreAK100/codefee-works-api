use crate::app::{db::PostgresPool, schema::todos, todo::models::Todo};
use crate::diesel::RunQueryDsl;
use actix_web::{get, web, HttpResponse, Responder};

#[get("")]
pub async fn execute(db_pool: web::Data<PostgresPool>) -> impl Responder {
    match db_pool.get() {
        Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
        con => HttpResponse::Ok().json(
            todos::table
                .load::<Todo>(&mut con.unwrap())
                .map(|r| r)
                .unwrap(),
        ),
    }
}
